use crisol_catalog::WidgetCatalog;
use crisol_catalog::schema::ToolboxGroup;

#[test]
fn every_kind_is_unique() {
    let catalog = WidgetCatalog::standard();
    let mut kinds = Vec::new();
    for widget in catalog.widgets() {
        assert!(!kinds.contains(&widget.kind), "{} twice", widget.kind);
        kinds.push(widget.kind);
    }
}

#[test]
fn every_default_value_fits_its_property_type() {
    for widget in WidgetCatalog::standard().widgets() {
        for property in &widget.properties {
            assert!(
                property.value_type.accepts(&property.default_value),
                "{}.{}",
                widget.kind,
                property.name
            );
        }
    }
}

#[test]
fn every_default_event_exists() {
    for widget in WidgetCatalog::standard().widgets() {
        if let Some(event) = widget.default_event {
            assert!(widget.event(event).is_some(), "{}.{event}", widget.kind);
        }
    }
}

#[test]
fn the_toolbox_offers_controls_and_containers_but_not_the_form() {
    let catalog = WidgetCatalog::standard();
    assert_eq!(catalog.in_group(ToolboxGroup::CommonControls).len(), 6);
    assert_eq!(catalog.in_group(ToolboxGroup::Containers).len(), 4);
    assert_eq!(catalog.in_group(ToolboxGroup::Hidden)[0].kind, "Form");
}

#[test]
fn only_the_absolute_panel_is_desktop_only() {
    for widget in WidgetCatalog::standard().widgets() {
        assert_eq!(
            widget.desktop_only,
            widget.kind == "Panel",
            "{}",
            widget.kind
        );
    }
}
