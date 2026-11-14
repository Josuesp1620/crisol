use std::rc::Rc;

use crisol_catalog::WidgetCatalog;
use crisol_ui::theme::Theme;
use gpui::{
    Context, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, div,
};

use super::{BottomTab, CanvasInteraction, ExplorerEntry, PropertyEditor, PropertyTab};
use crate::actions::{
    CloseMenu, DeleteSelection, DesignerKeyBindings, GenerateCode, NewForm, OpenForm, Quit, Redo,
    RunPreview, SaveForm, SaveFormAs, ToggleTheme, Undo,
};
use crate::designer_settings::DesignerSettings;
use crate::panels::{DocumentArea, SidePanel, StatusBar, TitleBar, ToolBar, ToolboxPanel};
use crate::session::{DocumentSession, OutputLog};

/// The designer window. It owns the open documents and the state of every panel; the
/// panels only read it and call back into it.
pub struct DesignerWorkspace {
    pub(super) catalog: Rc<WidgetCatalog>,
    pub(super) settings: DesignerSettings,
    pub(super) sessions: Vec<DocumentSession>,
    pub(super) active: usize,
    pub(super) output: OutputLog,
    pub(super) bottom_tab: BottomTab,
    pub(super) property_tab: PropertyTab,
    pub(super) focused_property: Option<super::PropertyKey>,
    pub(super) property_editor: Option<PropertyEditor>,
    pub(super) open_menu: Option<usize>,
    pub(super) interaction: CanvasInteraction,
    pub(super) toolbox_hover: Option<crisol_engine::dragging::DropTarget>,
    pub(super) explorer: Vec<ExplorerEntry>,
    focus_handle: FocusHandle,
}

impl Focusable for DesignerWorkspace {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DesignerWorkspace {
    pub fn new(first: DocumentSession, catalog: Rc<WidgetCatalog>, cx: &mut Context<Self>) -> Self {
        let mut workspace = Self {
            catalog,
            settings: DesignerSettings::default(),
            sessions: vec![first],
            active: 0,
            output: OutputLog::default(),
            bottom_tab: BottomTab::Errors,
            property_tab: PropertyTab::Properties,
            focused_property: None,
            property_editor: None,
            open_menu: None,
            interaction: CanvasInteraction::Idle,
            toolbox_hover: None,
            explorer: Vec::new(),
            focus_handle: cx.focus_handle(),
        };
        workspace.refresh_explorer();
        workspace.output.info("Crisol is ready.");
        workspace
    }

    pub fn settings(&self) -> &DesignerSettings {
        &self.settings
    }

    pub fn catalog(&self) -> &Rc<WidgetCatalog> {
        &self.catalog
    }

    pub fn sessions(&self) -> &[DocumentSession] {
        &self.sessions
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn session(&self) -> &DocumentSession {
        &self.sessions[self.active]
    }

    pub(super) fn session_mut(&mut self) -> &mut DocumentSession {
        &mut self.sessions[self.active]
    }

    pub fn output(&self) -> &OutputLog {
        &self.output
    }

    pub fn bottom_tab(&self) -> BottomTab {
        self.bottom_tab
    }

    pub fn property_tab(&self) -> PropertyTab {
        self.property_tab
    }

    pub fn focused_property(&self) -> Option<&super::PropertyKey> {
        self.focused_property.as_ref()
    }

    pub fn property_editor(&self) -> Option<&PropertyEditor> {
        self.property_editor.as_ref()
    }

    pub fn open_menu(&self) -> Option<usize> {
        self.open_menu
    }

    pub fn interaction(&self) -> &CanvasInteraction {
        &self.interaction
    }

    pub fn toolbox_hover(&self) -> Option<&crisol_engine::dragging::DropTarget> {
        self.toolbox_hover.as_ref()
    }

    pub fn explorer(&self) -> &[ExplorerEntry] {
        &self.explorer
    }

    pub fn show_bottom_tab(&mut self, tab: BottomTab, cx: &mut Context<Self>) {
        self.bottom_tab = tab;
        cx.notify();
    }

    pub fn show_property_tab(&mut self, tab: PropertyTab, cx: &mut Context<Self>) {
        self.property_tab = tab;
        self.property_editor = None;
        cx.notify();
    }

    pub fn activate(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.sessions.len() {
            self.active = index;
            self.property_editor = None;
            self.interaction = CanvasInteraction::Idle;
            self.refresh_explorer();
            cx.notify();
        }
    }
}

impl Render for DesignerWorkspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .key_context(DesignerKeyBindings::CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(
                cx.listener(|workspace, _: &NewForm, window, cx| workspace.new_form(window, cx)),
            )
            .on_action(
                cx.listener(|workspace, _: &OpenForm, window, cx| workspace.open_form(window, cx)),
            )
            .on_action(
                cx.listener(|workspace, _: &SaveForm, window, cx| workspace.save_form(window, cx)),
            )
            .on_action(cx.listener(|workspace, _: &SaveFormAs, window, cx| {
                workspace.save_form_as(window, cx);
            }))
            .on_action(
                cx.listener(|workspace, _: &GenerateCode, _, cx| workspace.generate_code(cx)),
            )
            .on_action(cx.listener(|workspace, _: &Undo, _, cx| workspace.undo(cx)))
            .on_action(cx.listener(|workspace, _: &Redo, _, cx| workspace.redo(cx)))
            .on_action(
                cx.listener(|workspace, _: &DeleteSelection, _, cx| workspace.delete_selection(cx)),
            )
            .on_action(cx.listener(|workspace, _: &RunPreview, _, cx| workspace.run_preview(cx)))
            .on_action(cx.listener(|workspace, _: &ToggleTheme, _, cx| workspace.toggle_theme(cx)))
            .on_action(cx.listener(|workspace, _: &CloseMenu, _, cx| workspace.close_menu(cx)))
            .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
            .size_full()
            .flex()
            .flex_col()
            .relative()
            .bg(theme.colors.surface)
            .text_color(theme.colors.ink)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.metrics.text_size)
            .child(TitleBar::new(self).render(window, cx))
            .child(ToolBar::new(self).render(window, cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .child(ToolboxPanel::new(self).render(window, cx))
                    .child(DocumentArea::new(self).render(window, cx))
                    .child(SidePanel::new(self).render(window, cx)),
            )
            .child(StatusBar::new(self).render(window, cx))
    }
}
