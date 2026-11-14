//! Every control Crisol knows, described once: its properties, events, defaults and design
//! rules. The property grid, validation, the toolbox and code generation all read from here.

pub mod naming;
pub mod property_reader;
pub mod schema;
pub mod validation;
pub mod widget_catalog;
pub mod widgets;

pub use property_reader::PropertyReader;
pub use widget_catalog::WidgetCatalog;
