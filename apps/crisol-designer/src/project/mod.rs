//! New projects: templates, the files they start with and the list of recent ones.

mod cargo_runner;
mod forms_module_writer;
mod project_scaffolder;
mod project_session;
mod project_template;
mod recent_projects;

pub use cargo_runner::CargoRunner;
pub use forms_module_writer::FormsModuleWriter;
pub use project_scaffolder::ProjectScaffolder;
pub use project_session::ProjectSession;
pub use project_template::ProjectTemplate;
pub use recent_projects::RecentProjects;
