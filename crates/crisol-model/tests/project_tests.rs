use crisol_model::project::{ProjectManifest, ProjectSerializer};

#[test]
fn the_first_form_starts_the_application() {
    let mut manifest = ProjectManifest::new("billing");
    manifest.add_form("src/forms/main_form.crisol");
    manifest.add_form("src/forms/customer.crisol");
    manifest.add_form("src/forms/customer.crisol");
    assert_eq!(manifest.forms.len(), 2);
    assert_eq!(
        manifest.startup_form.as_deref(),
        Some("src/forms/main_form.crisol")
    );
}

#[test]
fn removing_the_startup_form_picks_the_next_one() {
    let mut manifest = ProjectManifest::new("billing");
    manifest.add_form("a.crisol");
    manifest.add_form("b.crisol");
    manifest.remove_form("a.crisol");
    assert_eq!(manifest.startup_form.as_deref(), Some("b.crisol"));
}

#[test]
fn manifests_round_trip_with_their_format() {
    let mut manifest = ProjectManifest::new("billing");
    manifest.add_form("src/forms/main_form.crisol");
    let text = ProjectSerializer::write(&manifest).unwrap();
    assert!(text.contains("\"format\": \"crisol-project/1\""));
    assert_eq!(ProjectSerializer::read(&text).unwrap(), manifest);
    assert!(
        ProjectSerializer::read(&text.replace("crisol-project/1", "crisol-project/9")).is_err()
    );
}
