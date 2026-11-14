//! Forms designed in Crisol. `*.generated.rs` files are rewritten by `crisol-cli generate`;
//! the `*_events.rs` files are ours. Generated files keep the generator's layout.

#[rustfmt::skip]
#[path = "customer_form.generated.rs"]
pub mod customer_form;
mod customer_form_events;
