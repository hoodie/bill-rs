use super::{BillItem, BillProduct, Money};
use std::ops::Deref;

/// A list of `BillItem`s, implements summing methods.
#[derive(Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize))]
pub struct ItemList<P> {
    items: Vec<BillItem<P>>,
}

impl<P: BillProduct> ItemList<P> {
    pub fn from_vec(list: Vec<BillItem<P>>) -> Self {
        ItemList { items: list }
    }

    pub fn new() -> Self {
        ItemList { items: Vec::new() }
    }

    pub fn gross_sum(&self) -> Money {
        self.items
            .iter()
            .map(|i| i.gross())
            .fold(Money::default(), |acc, x| acc + x)
    }

    /// this assumes that all items have the same tax
    pub fn tax_sum(&self) -> Money {
        if let Some(tax) = self.items.get(0).map(|i| i.product.tax()) {
            self.gross_sum() * **tax
        } else {
            Money::default()
        }
    }

    pub fn net_sum(&self) -> Money {
        self.gross_sum() + self.tax_sum()
    }

    pub fn push(&mut self, item: BillItem<P>) {
        self.items.push(item)
    }
}


impl<P: BillProduct> Deref for ItemList<P> {
    type Target = [BillItem<P>];
    fn deref(&self) -> &[BillItem<P>] {
        self.items.deref()
    }
}
