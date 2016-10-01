//! Product and Invoicing primitives.
//!
//! There is no difference between Invoices and Offers, so here there are only `Bill`s.
//! If you need to have Offers and invoices use two different `Bill`s.

extern crate ordered_float;
extern crate claude;

use ordered_float::OrderedFloat;
pub use claude::Currency;

use std::collections::BTreeMap;
use std::ops::Deref;

mod itemlist;
pub use itemlist::ItemList;

mod product;
pub use product::{Product, BillProduct};

mod item;
pub use item::BillItem;

/// Representation of Tax value
pub type Tax = OrderedFloat<f64>;
pub type Amount = f64;
pub type Money = Currency;


/// This is where the magic happens.
#[derive(Debug)]
pub struct Bill<P> {
    pub items_by_tax: BTreeMap<Tax, ItemList<P>>,
}

impl<P: BillProduct> Deref for Bill<P> {
    type Target = BTreeMap<Tax, ItemList<P>>;
    fn deref(&self) -> &BTreeMap<Tax, ItemList<P>> {
        &self.items_by_tax
    }
}

impl<P:BillProduct> Bill<P> {
    /// Instantiates a new `Bill`
    pub fn new() -> Self {
        Bill { items_by_tax: BTreeMap::new() }
    }

    pub fn to_items_with_tax(&self) -> BTreeMap<Tax, &BillItem<P>> {
        self.as_items_with_tax().into_iter().collect()
    }

    /// TODO Make this return an `Iterator`
    pub fn as_items_with_tax(&self) -> Vec<(Tax, &BillItem<P>)> {
        let mut out = Vec::new();
        for (tax, items) in self.items_by_tax.iter().rev() {
            for item in items.iter(){
                out.push((tax.to_owned(), item));
            }
        }
        out
    }

    pub fn items_by_tax(&self) -> BTreeMap<Tax, ItemList<P>> {
        unimplemented!()
    }


    /// TODO Make this return an `Iterator`
    pub fn as_items(&self) -> Vec<&BillItem<P>> {
        self.as_items_with_tax().into_iter().map(|(_,item)|item).collect()
    }

    /// Adds a new `BillItem` to the list.
    pub fn add(&mut self, item:BillItem<P>) {
        let tax = item.product.tax();
        self.items_by_tax.entry(tax).or_insert_with(ItemList::new).push(item);
    }

    /// Instantiates and adds a new `BillItem` to the list.
    pub fn add_item(&mut self, amount: Amount, product: P) {
        let item = BillItem {
            amount: amount,
            product: product,
        };

        self.add(item)
    }

    pub fn sums_by_tax(&self) -> BTreeMap<Tax, Currency> {
        self.items_by_tax
            .iter()
            .map(|(tax, items)| (*tax, items.gross_sum()) )
            .collect()
    }

    pub fn gross_total(&self) -> Money {
        self.items_by_tax
            .iter()
            .map(|(_, items)| items.gross_sum())
        .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn tax_total(&self) -> Money {
        self.items_by_tax
            .iter()
            .map(|(_, items)| items.tax_sum())
        .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn net_total(&self) -> Money {
        self.items_by_tax
            .iter()
            .map(|(_, items)| items.net_sum())
        .fold(Money::default(), |acc, x| acc + x)
    }

}
