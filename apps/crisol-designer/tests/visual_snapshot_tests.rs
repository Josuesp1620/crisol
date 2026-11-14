use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crisol_catalog::WidgetCatalog;
use crisol_designer::actions::DesignerKeyBindings;
use crisol_designer::designer_application::DesignerApplication;
use crisol_designer::workspace::DesignerWorkspace;
use crisol_ui::CrisolUi;
use crisol_ui::assets::CrisolAssets;
use crisol_ui::theme::ThemeMode;
use gpui::{AppContext, HeadlessAppContext, px, size};

/// Renders the designer with the GPU and saves `target/screenshots/designer-<mode>.png`.
struct DesignerSnapshot;

impl DesignerSnapshot {
    fn capture(mode: ThemeMode, file_name: &str) -> PathBuf {
        Self::capture_with(mode, file_name, None)
    }

    /// Captures after selecting the control named `selected`, to show handles and its properties.
    fn capture_with(mode: ThemeMode, file_name: &str, selected: Option<&str>) -> PathBuf {
        let text_system = Arc::new(gpui_wgpu::CosmicTextSystem::new("Instrument Sans"));
        let mut cx = HeadlessAppContext::with_platform(
            text_system,
            Arc::new(CrisolAssets),
            gpui_platform::current_headless_renderer,
        );
        cx.update(|cx| {
            CrisolUi::install(mode, cx).expect("the bundled fonts load");
            DesignerKeyBindings::install(cx);
        });
        let catalog = Rc::new(WidgetCatalog::standard());
        let session = DesignerApplication::sample_session(&catalog);
        let window = cx
            .open_window(size(px(1440.), px(900.)), |_, cx| {
                cx.new(|cx| DesignerWorkspace::new(session, catalog, cx))
            })
            .expect("the window opens");
        cx.run_until_parked();
        if let Some(name) = selected {
            let workspace = window.root(&mut cx).expect("the workspace is the root");
            cx.update(|cx| {
                workspace.update(cx, |workspace, cx| {
                    let node = workspace
                        .session()
                        .document()
                        .find_by_name(name)
                        .expect("the control exists");
                    workspace.select(node, false, cx);
                });
            });
        }
        cx.run_until_parked();
        let _ = cx.capture_screenshot(window.into());
        cx.run_until_parked();
        let image = cx
            .capture_screenshot(window.into())
            .expect("a GPU renderer is available");
        let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/screenshots");
        std::fs::create_dir_all(&folder).expect("the folder is writable");
        let path = folder.join(file_name);
        image.save(&path).expect("the screenshot is saved");
        path
    }
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn the_designer_renders_in_light_mode() {
    assert!(DesignerSnapshot::capture(ThemeMode::Light, "designer-light.png").exists());
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn the_designer_renders_in_dark_mode() {
    assert!(DesignerSnapshot::capture(ThemeMode::Dark, "designer-dark.png").exists());
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn a_selected_control_shows_its_handles_and_properties() {
    assert!(
        DesignerSnapshot::capture_with(
            ThemeMode::Light,
            "designer-selection.png",
            Some("cmb_kind")
        )
        .exists()
    );
}
