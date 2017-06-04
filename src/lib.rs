//! Product and Invoicing primitives.
//!
//! Please check out `examples/catalogue.rs`

extern crate ordered_float;
extern crate claude;
#[cfg(feature="serialization")] extern crate serde;
#[cfg(feature="serialization")] extern crate serde_json;
#[cfg(feature="serialization")] #[macro_use] extern crate serde_derive;

pub use claude::Currency;

mod bill;
pub use bill::Bill;

mod itemlist;
pub use itemlist::ItemList;

mod product;
pub use product::{Product, BillProduct};

mod item;
pub use item::BillItem;

mod tax;
pub use tax::Tax;

/// Representation of Item quantities.
pub type Amount = f64;
/// Alias for `Currency`.
pub type Money = Currency;

