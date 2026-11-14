use std::fs;
use std::path::PathBuf;

use crisol_catalog::WidgetCatalog;
use crisol_designer::project::{ProjectScaffolder, ProjectSession, ProjectTemplate};
use crisol_designer::session::DocumentFiles;
use crisol_ui::theme::ThemeMode;

/// A fresh folder under the system's temporary folder, removed at the end.
struct ScratchFolder {
    path: PathBuf,
}

impl ScratchFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("crisol-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for ScratchFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn a_new_project_has_a_manifest_a_main_form_and_its_code() {
    let folder = ScratchFolder::new("scaffold");
    let catalog = WidgetCatalog::standard();
    let manifest = ProjectScaffolder::new(&catalog)
        .create(
            "billing app",
            &folder.path,
            ProjectTemplate::DataForm,
            ThemeMode::Dark,
        )
        .unwrap();
    let root = folder.path.join("billing app");
    assert_eq!(manifest, root.join("billing-app.crisolproj"));
    for file in [
        "Cargo.toml",
        "rust-toolchain.toml",
        "src/main.rs",
        "src/forms/mod.rs",
        "src/forms/main_form.crisol",
        "src/forms/main_form.generated.rs",
        "src/forms/main_form_events.rs",
    ] {
        assert!(root.join(file).exists(), "{file} is missing");
    }
    let project = ProjectSession::load(&manifest).unwrap();
    assert_eq!(
        project.startup_form(),
        Some(root.join("src/forms/main_form.crisol"))
    );
    let form = DocumentFiles::load(&root.join("src/forms/main_form.crisol")).unwrap();
    assert_eq!(form.form().name.as_str(), "MainForm");
    let modules = fs::read_to_string(root.join("src/forms/mod.rs")).unwrap();
    assert!(modules.contains("pub mod main_form;"));
    assert!(
        fs::read_to_string(root.join("src/main.rs"))
            .unwrap()
            .contains("ThemeMode::Dark")
    );
    assert!(
        fs::read_to_string(root.join("Cargo.toml"))
            .unwrap()
            .contains("name = \"billing-app\"")
    );
}

#[test]
fn an_existing_folder_is_never_overwritten() {
    let folder = ScratchFolder::new("existing");
    fs::create_dir_all(folder.path.join("taken")).unwrap();
    let catalog = WidgetCatalog::standard();
    let result = ProjectScaffolder::new(&catalog).create(
        "taken",
        &folder.path,
        ProjectTemplate::Empty,
        ThemeMode::Light,
    );
    assert!(result.is_err());
}

#[test]
fn names_must_start_with_a_letter() {
    let folder = ScratchFolder::new("names");
    let catalog = WidgetCatalog::standard();
    let result = ProjectScaffolder::new(&catalog).create(
        "1app",
        &folder.path,
        ProjectTemplate::Empty,
        ThemeMode::Light,
    );
    assert!(result.is_err());
}

#[test]
fn every_template_produces_a_valid_form() {
    let catalog = WidgetCatalog::standard();
    for template in ProjectTemplate::ALL {
        let document = template.main_form("Demo", &catalog);
        let errors = crisol_catalog::validation::CatalogValidator::new(&catalog)
            .validate(&document)
            .into_iter()
            .filter(crisol_catalog::validation::Diagnostic::is_error)
            .count();
        assert_eq!(errors, 0, "{}", template.label());
    }
}
