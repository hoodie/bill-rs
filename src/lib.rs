//! Product and Invoicing primitives.
//!
//! Please check out `examples/catalogue.rs`

pub use claude::Currency;

mod bill;
pub use crate::bill::Bill;

mod itemlist;
pub use crate::itemlist::ItemList;

mod product;
pub use crate::product::{BillProduct, Product};

mod item;
pub use crate::item::BillItem;

mod tax;
pub use crate::tax::Tax;

/// Representation of Item quantities.
pub type Amount = f64;
/// Alias for `Currency`.
pub type Money = Currency;
