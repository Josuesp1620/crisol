//! Forms designed in Crisol. `*.generated.rs` files are rewritten by `crisol-cli generate`;
//! the `*_events.rs` files are ours. Generated files keep the generator's layout.

#[rustfmt::skip]
#[path = "customer.generated.rs"]
pub mod customer;
mod customer_events;
