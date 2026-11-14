#![cfg(target_os = "linux")]

use std::rc::Rc;
use std::sync::Arc;

use crisol_catalog::WidgetCatalog;
use crisol_designer::actions::DesignerKeyBindings;
use crisol_designer::designer_application::DesignerApplication;
use crisol_designer::workspace::{DesignerWorkspace, PropertyKey};
use crisol_model::geometry::Rect;
use crisol_model::identity::NodeId;
use crisol_ui::CrisolUi;
use crisol_ui::assets::CrisolAssets;
use crisol_ui::theme::ThemeMode;
use gpui::{
    AppContext, Entity, HeadlessAppContext, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PlatformInput, Point, WindowHandle, point, px, size,
};

/// The designer in a window without a screen, driven with real mouse events.
struct DesignerHarness {
    cx: HeadlessAppContext,
    window: WindowHandle<DesignerWorkspace>,
    workspace: Entity<DesignerWorkspace>,
}

impl DesignerHarness {
    fn open() -> Self {
        let text_system = Arc::new(gpui_wgpu::CosmicTextSystem::new("Instrument Sans"));
        let mut cx = HeadlessAppContext::with_asset_source(text_system, Arc::new(CrisolAssets));
        cx.update(|cx| {
            CrisolUi::install(ThemeMode::Light, cx).expect("the bundled fonts load");
            DesignerKeyBindings::install(cx);
        });
        let catalog = Rc::new(WidgetCatalog::standard());
        let session = DesignerApplication::sample_session(&catalog);
        let window = cx
            .open_window(size(px(1440.), px(900.)), |_, cx| {
                cx.new(|cx| DesignerWorkspace::new(session, false, catalog, cx))
            })
            .expect("the window opens");
        let workspace = window.root(&mut cx).expect("the workspace is the root");
        let mut harness = Self {
            cx,
            window,
            workspace,
        };
        harness.draw();
        harness
    }

    fn draw(&mut self) {
        self.cx.run_until_parked();
        self.cx
            .update_window(self.window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
            })
            .expect("the window draws");
        self.cx.run_until_parked();
    }

    fn node(&mut self, name: &str) -> NodeId {
        self.cx.update(|cx| {
            self.workspace
                .read(cx)
                .session()
                .document()
                .find_by_name(name)
                .expect("the control exists")
        })
    }

    fn bounds(&mut self, node: NodeId) -> Rect {
        self.cx.update(|cx| {
            self.workspace
                .read(cx)
                .session()
                .recorder()
                .snapshot()
                .bounds_of(node)
                .expect("the control was drawn")
        })
    }

    fn center(&mut self, name: &str) -> Point<gpui::Pixels> {
        let node = self.node(name);
        let rect = self.bounds(node);
        point(px(rect.center_x() as f32), px(rect.center_y() as f32))
    }

    fn selected(&mut self) -> Option<NodeId> {
        self.cx
            .update(|cx| self.workspace.read(cx).session().selection().primary())
    }

    fn children_of(&mut self, name: &str) -> Vec<String> {
        self.cx.update(|cx| {
            let document = self.workspace.read(cx).session().document();
            let parent = document.find_by_name(name).expect("the container exists");
            let mut names = Vec::new();
            for child in document.children_of(parent) {
                names.push(document.node(*child).expect("a child").name.to_string());
            }
            names
        })
    }

    fn send(&mut self, input: PlatformInput) {
        self.cx
            .update_window(self.window.into(), |_, window, cx| {
                window.dispatch_event(input, cx)
            })
            .expect("the window takes input");
        self.draw();
    }

    fn press(&mut self, position: Point<gpui::Pixels>) {
        self.send(PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::default(),
            click_count: 1,
            first_mouse: false,
        }));
    }

    fn move_pressed(&mut self, position: Point<gpui::Pixels>) {
        self.send(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: Some(MouseButton::Left),
            modifiers: Modifiers::default(),
        }));
    }

    fn release(&mut self, position: Point<gpui::Pixels>) {
        self.send(PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::default(),
            click_count: 1,
        }));
    }

    fn click(&mut self, position: Point<gpui::Pixels>) {
        self.press(position);
        self.release(position);
    }

    fn undo(&mut self) {
        self.cx.update(|cx| {
            self.workspace
                .update(cx, crisol_designer::workspace::DesignerWorkspace::undo);
        });
        self.draw();
    }
}

#[test]
fn clicking_a_control_selects_it_and_clicking_the_canvas_selects_the_form() {
    let mut harness = DesignerHarness::open();
    let kind = harness.node("cmb_kind");
    let center = harness.center("cmb_kind");
    harness.click(center);
    assert_eq!(harness.selected(), Some(kind));

    let root = harness.node("frm_customer");
    harness.click(point(px(1500.) / 2., px(1100.) / 2.));
    assert_eq!(harness.selected(), Some(root));
}

#[test]
fn dragging_a_button_past_its_neighbour_reorders_the_row_and_undo_restores_it() {
    let mut harness = DesignerHarness::open();
    assert_eq!(
        harness.children_of("stk_buttons"),
        vec!["btn_cancel", "btn_save"]
    );
    let start = harness.center("btn_cancel");
    let save = harness.node("btn_save");
    let save_bounds = harness.bounds(save);
    let target = point(px(save_bounds.right() as f32 - 4.), start.y);
    harness.press(start);
    harness.move_pressed(point(start.x + px(10.), start.y));
    harness.move_pressed(target);
    harness.release(target);
    assert_eq!(
        harness.children_of("stk_buttons"),
        vec!["btn_save", "btn_cancel"]
    );

    harness.undo();
    assert_eq!(
        harness.children_of("stk_buttons"),
        vec!["btn_cancel", "btn_save"]
    );
}

#[test]
fn deleting_the_selection_removes_the_control() {
    let mut harness = DesignerHarness::open();
    let center = harness.center("chk_active");
    harness.click(center);
    harness.cx.update(|cx| {
        harness.workspace.update(
            cx,
            crisol_designer::workspace::DesignerWorkspace::delete_selection,
        );
    });
    harness.draw();
    assert!(
        !harness
            .children_of("grd_fields")
            .contains(&"chk_active".to_owned())
    );
}

#[test]
fn adding_from_the_toolbox_appends_to_the_selected_container() {
    let mut harness = DesignerHarness::open();
    let row = harness.node("stk_buttons");
    harness.cx.update(|cx| {
        harness.workspace.update(cx, |workspace, cx| {
            workspace.select(row, false, cx);
            workspace.add_control_to_selection("Button", cx);
        });
    });
    harness.draw();
    assert_eq!(
        harness.children_of("stk_buttons"),
        vec!["btn_cancel", "btn_save", "btn_1"]
    );
    let added = harness.node("btn_1");
    assert_eq!(harness.selected(), Some(added));
}

#[test]
fn clicking_a_yes_no_property_flips_it() {
    let mut harness = DesignerHarness::open();
    let check = harness.node("chk_active");
    let window = harness.window;
    harness
        .cx
        .update_window(window.into(), |_, window, cx| {
            harness.workspace.update(cx, |workspace, cx| {
                workspace.select(check, false, cx);
                workspace.begin_edit(&PropertyKey::Property("checked".to_owned()), window, cx);
            });
        })
        .expect("the window takes the edit");
    let checked = harness.cx.update(|cx| {
        let document = harness.workspace.read(cx).session().document();
        document
            .node(check)
            .expect("the control exists")
            .properties
            .get("checked")
            .cloned()
    });
    assert_eq!(
        checked,
        Some(crisol_model::properties::PropertyValue::Boolean(false))
    );
}

#[test]
fn dragging_the_form_corner_resizes_the_form() {
    let mut harness = DesignerHarness::open();
    let root = harness.node("frm_customer");
    harness.cx.update(|cx| {
        harness
            .workspace
            .update(cx, |workspace, cx| workspace.select(root, false, cx));
    });
    harness.draw();
    let bounds = harness.bounds(root);
    let corner = point(px(bounds.right() as f32), px(bounds.bottom() as f32));
    let target = point(corner.x + px(40.), corner.y + px(30.));
    harness.press(corner);
    harness.move_pressed(point(corner.x + px(10.), corner.y + px(10.)));
    harness.move_pressed(target);
    harness.release(target);
    let size = harness
        .cx
        .update(|cx| harness.workspace.read(cx).session().document().form().size);
    assert_eq!(
        (size.width, size.height),
        (bounds.size.width + 40, bounds.size.height + 30)
    );
}
