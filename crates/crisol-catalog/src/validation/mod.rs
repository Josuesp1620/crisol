//! Checks a document against the catalog and reports what is wrong or not portable.

mod accessibility_rule;
mod catalog_validator;
mod containment_rule;
mod diagnostic;
mod diagnostic_rule;
mod diagnostic_severity;
mod event_rule;
mod kind_rule;
mod portability_rule;
mod property_rule;

pub use catalog_validator::CatalogValidator;
pub use diagnostic::Diagnostic;
pub use diagnostic_rule::{DiagnosticRule, RuleContext};
pub use diagnostic_severity::DiagnosticSeverity;
