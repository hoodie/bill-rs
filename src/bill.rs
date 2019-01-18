#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Money, Amount, ItemList, BillItem, BillProduct};
use crate::tax::Tax;

use std::collections::BTreeMap;
use std::ops::Deref;

/// This is where the magic happens.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Bill<P> {
    pub items_by_tax: BTreeMap<Tax, ItemList<P>>,
}

impl<P: BillProduct> Deref for Bill<P> {
    type Target = BTreeMap<Tax, ItemList<P>>;
    fn deref(&self) -> &BTreeMap<Tax, ItemList<P>> {
        &self.items_by_tax
    }
}

impl<P: BillProduct> Bill<P> {
    /// Instantiates a new `Bill`
    pub fn new() -> Self {
        Bill { items_by_tax: BTreeMap::new() }
    }

    pub fn to_items_with_tax(&self) -> BTreeMap<Tax, &BillItem<P>> {
        self.as_items_with_tax().into_iter().collect()
    }

    /// Returns a `Vec` of `Tax` and `BillItem`
    pub fn as_items_with_tax(&self) -> Vec<(Tax, &BillItem<P>)> {
        let mut out = Vec::new();
        for (tax, items) in self.items_by_tax.iter().rev() {
            for item in items.iter() {
                out.push((tax.to_owned(), item));
            }
        }
        out
    }

    /// Returns a `Vec` of `BillItem`
    pub fn as_items(&self) -> Vec<&BillItem<P>> {
        self.as_items_with_tax().into_iter().map(|(_, item)| item).collect()
    }

    /// Adds a new `BillItem` to the list.
    pub fn add(&mut self, item: BillItem<P>) {
        let tax = item.product.tax();
        self.items_by_tax.entry(tax).or_insert_with(ItemList::new).push(item);
    }

    /// Instantiates and adds a new `BillItem` to the list.
    pub fn add_item(&mut self, amount: Amount, product: P) {
        let item = BillItem {
            amount,
            product,
        };

        self.add(item)
    }

    pub fn sums_by_tax(&self) -> BTreeMap<Tax, Money> {
        self.items_by_tax
            .iter()
            .map(|(tax, items)| (*tax, items.gross_sum()))
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
