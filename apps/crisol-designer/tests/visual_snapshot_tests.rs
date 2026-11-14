use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crisol_catalog::WidgetCatalog;
use crisol_designer::actions::DesignerKeyBindings;
use crisol_designer::designer_application::DesignerApplication;
use crisol_designer::project::{ProjectScaffolder, ProjectTemplate};
use crisol_designer::workspace::DesignerWorkspace;
use crisol_ui::CrisolUi;
use crisol_ui::assets::CrisolAssets;
use crisol_ui::theme::ThemeMode;
use gpui::{AppContext, Context, HeadlessAppContext, px, size};

/// Renders the designer with the GPU and saves `target/screenshots/<name>.png`.
struct DesignerSnapshot;

impl DesignerSnapshot {
    fn capture(
        mode: ThemeMode,
        file_name: &str,
        show_start: bool,
        prepare: impl FnOnce(&mut DesignerWorkspace, &mut Context<DesignerWorkspace>),
    ) -> PathBuf {
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
                cx.new(|cx| DesignerWorkspace::new(session, show_start, catalog, cx))
            })
            .expect("the window opens");
        cx.run_until_parked();
        let workspace = window.root(&mut cx).expect("the workspace is the root");
        cx.update(|cx| workspace.update(cx, prepare));
        cx.run_until_parked();
        let _ = cx.capture_screenshot(window.into());
        cx.run_until_parked();
        let image = cx
            .capture_screenshot(window.into())
            .expect("a GPU renderer is available");
        let folder = Self::folder();
        std::fs::create_dir_all(&folder).expect("the folder is writable");
        let path = folder.join(file_name);
        image.save(&path).expect("the screenshot is saved");
        path
    }

    fn folder() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/screenshots")
    }
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn the_designer_renders_in_light_mode() {
    assert!(
        DesignerSnapshot::capture(ThemeMode::Light, "designer-light.png", false, |_, _| {})
            .exists()
    );
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn the_designer_renders_in_dark_mode() {
    assert!(
        DesignerSnapshot::capture(ThemeMode::Dark, "designer-dark.png", false, |_, _| {}).exists()
    );
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn a_selected_control_shows_its_handles_and_properties() {
    let path = DesignerSnapshot::capture(
        ThemeMode::Light,
        "designer-selection.png",
        false,
        |workspace, cx| {
            let node = workspace
                .session()
                .document()
                .find_by_name("cmb_kind")
                .expect("the control exists");
            workspace.select(node, false, cx);
        },
    );
    assert!(path.exists());
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn the_start_page_and_the_new_project_dialog_render() {
    assert!(
        DesignerSnapshot::capture(ThemeMode::Light, "designer-start.png", true, |_, _| {}).exists()
    );
    let path = DesignerSnapshot::capture(
        ThemeMode::Light,
        "designer-new-project.png",
        true,
        |workspace, cx| {
            workspace.open_new_project_without_window(cx);
        },
    );
    assert!(path.exists());
}

#[test]
#[ignore = "needs a GPU or a software Vulkan driver; run with make screenshots"]
fn a_project_opens_with_its_explorer_and_code() {
    let folder = DesignerSnapshot::folder().join("project");
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("the folder is writable");
    let catalog = WidgetCatalog::standard();
    let manifest = ProjectScaffolder::new(&catalog)
        .create(
            "billing",
            &folder,
            ProjectTemplate::DataForm,
            ThemeMode::Light,
        )
        .expect("the project is created");
    let designer_path = DesignerSnapshot::capture(
        ThemeMode::Light,
        "designer-project.png",
        true,
        |workspace, cx| {
            workspace.open_project(manifest.clone(), cx);
        },
    );
    let code_path = DesignerSnapshot::capture(
        ThemeMode::Light,
        "designer-code.png",
        true,
        |workspace, cx| {
            workspace.open_project(manifest.clone(), cx);
            workspace.view_code(Some("save"), cx);
        },
    );
    assert!(designer_path.exists() && code_path.exists());
}
