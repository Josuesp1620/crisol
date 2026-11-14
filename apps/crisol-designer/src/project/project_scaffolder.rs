use std::fs;
use std::path::{Path, PathBuf};

use crisol_catalog::WidgetCatalog;
use crisol_model::project::{ProjectManifest, ProjectSerializer};
use crisol_ui::theme::ThemeMode;

use super::{FormsModuleWriter, ProjectSession, ProjectTemplate};
use crate::session::{CodeExport, DocumentFiles};

/// Writes a new Cargo project ready to run: manifest, toolchain, main window and the main
/// form with its generated code.
pub struct ProjectScaffolder<'catalog> {
    catalog: &'catalog WidgetCatalog,
}

impl<'catalog> ProjectScaffolder<'catalog> {
    const REPOSITORY: &'static str = "https://github.com/Josuesp1620/crisol";
    const ZED_REPOSITORY: &'static str = "https://github.com/zed-industries/zed";
    const GPUI_REVISION: &'static str = "56a157e5b01b62afd5c50a014ed15b379de8dfd6";
    const FORM_FILE: &'static str = "main_form.crisol";

    pub fn new(catalog: &'catalog WidgetCatalog) -> Self {
        Self { catalog }
    }

    /// Creates `<location>/<name>` and returns the path of its `.crisolproj` file.
    pub fn create(
        &self,
        name: &str,
        location: &Path,
        template: ProjectTemplate,
        theme: ThemeMode,
    ) -> Result<PathBuf, String> {
        let crate_name = Self::crate_name(name)?;
        let root = location.join(name);
        if root.exists() {
            return Err(format!(
                "{} already exists. Choose another name or location.",
                root.display()
            ));
        }
        let forms = root.join("src").join("forms");
        fs::create_dir_all(&forms)
            .map_err(|error| format!("Cannot create {}: {error}", forms.display()))?;
        Self::write(&root.join("Cargo.toml"), &Self::manifest(&crate_name))?;
        Self::write(
            &root.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.98.1\"\nprofile = \"minimal\"\n",
        )?;
        Self::write(&root.join(".gitignore"), "/target\n")?;
        Self::write(
            &root.join("src").join("main.rs"),
            &Self::main_source(name, theme),
        )?;
        let form_path = forms.join(Self::FORM_FILE);
        let document = template.main_form(name, self.catalog);
        DocumentFiles::save(&document, &form_path)?;
        CodeExport::new(self.catalog).export(&document, &form_path)?;
        FormsModuleWriter::write(&forms)?;
        let manifest_path = root.join(format!("{crate_name}.{}", ProjectSerializer::EXTENSION));
        let mut project =
            ProjectSession::create(manifest_path.clone(), ProjectManifest::new(name))?;
        project.add_form(&form_path)?;
        Ok(manifest_path)
    }

    /// Cargo package names: lowercase letters, digits, `-` and `_`.
    fn crate_name(name: &str) -> Result<String, String> {
        let mut crate_name = String::with_capacity(name.len());
        for character in name.trim().chars() {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                crate_name.push(character.to_ascii_lowercase());
            } else if character == ' ' {
                crate_name.push('-');
            } else {
                return Err(format!("{name} has characters a project name cannot use."));
            }
        }
        if crate_name.is_empty()
            || !crate_name.starts_with(|first: char| first.is_ascii_alphabetic())
        {
            return Err("A project name starts with a letter.".to_owned());
        }
        Ok(crate_name)
    }

    fn manifest(crate_name: &str) -> String {
        let version = env!("CARGO_PKG_VERSION");
        let zed = Self::ZED_REPOSITORY;
        let revision = Self::GPUI_REVISION;
        format!(
            "[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
             # A project of its own, even when created inside another Cargo workspace.\n[workspace]\n\n[dependencies]\n\
             crisol-ui = {{ git = \"{repository}\", tag = \"v{version}\" }}\n\
             gpui = {{ git = \"{zed}\", rev = \"{revision}\" }}\n\
             gpui_platform = {{ git = \"{zed}\", rev = \"{revision}\", features = [\"x11\", \"wayland\"] }}\n\n\
             # GPUI needs the same fixes Zed applies to these crates.\n[patch.crates-io]\n\
             async-task = {{ git = \"https://github.com/smol-rs/async-task\", rev = \"41fd3359c75e221bd8b187a8ee361aa309c1714c\" }}\n\
             calloop = {{ git = \"https://github.com/zed-industries/calloop\", rev = \"3759371fee14c40066d64777e2a36b6ffcc22590\" }}\n",
            repository = Self::REPOSITORY,
        )
    }

    fn main_source(title: &str, theme: ThemeMode) -> String {
        let mode = match theme {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        };
        format!(
            r#"// A window application on Windows: no console window behind the form.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod forms;

use crisol_ui::prelude::*;
use gpui::{{App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size}};

use forms::main_form::MainForm;

/// {title}
fn main() {{
    gpui_platform::application().with_assets(CrisolAssets).run(|cx: &mut App| {{
        CrisolUi::install(ThemeMode::{mode}, cx).expect("the bundled fonts load");
        let (width, height) = MainForm::DESIGN_SIZE;
        let options = WindowOptions {{
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(width), px(height)), cx))),
            titlebar: Some(TitlebarOptions {{
                title: Some(MainForm::TITLE.into()),
                ..TitlebarOptions::default()
            }}),
            ..WindowOptions::default()
        }};
        cx.open_window(options, |window, cx| cx.new(|cx| MainForm::new(window, cx)))
            .expect("the window opens");
    }});
}}
"#
        )
    }

    fn write(path: &Path, content: &str) -> Result<(), String> {
        fs::write(path, content)
            .map_err(|error| format!("Cannot write {}: {error}", path.display()))
    }
}
