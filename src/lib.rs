//! Product and Invoicing primitives.
//!
//! There is no difference between Invoices and Offers, so here there are only `Bill`s.
//! If you need to have Offers and invoices use two different `Bill`s.

extern crate multimap;
extern crate ordered_float;
extern crate claude;

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;
use multimap::MultiMap;
pub use claude::Currency;

pub mod display;

/// Representation of Tax value
pub type Tax = OrderedFloat<f64>;
pub type Amount = f64;
pub type Money = Currency;


fn tax_from_f64(float: f64) -> Tax {
    OrderedFloat(float)
}

#[derive(Clone, Copy, Debug)]
pub struct Product<'a> {
    pub name: &'a str,
    // pub unit: Option<&'a str>,
    pub price: Money,
    pub tax: Tax,
}

impl<'a> Product<'a> {
    pub fn new(name: &'a str, price: Money, tax: f64) -> Self {
        Product {
            name: name,
            price: price,
            tax: tax_from_f64(tax),
        }
    }
}

#[derive(Debug)]
pub struct BillItem<'a> {
    pub amount: Amount,
    pub product: Product<'a>,
}

impl<'a> BillItem<'a> {

    pub fn sum(&self) -> Money {
        self.product.price * self.amount
    }

}

#[derive(Debug)]
pub struct Bill<'a> {
    pub items_by_tax: MultiMap<Tax, BillItem<'a>>,
}

impl<'a> Bill<'a> {
    pub fn new() -> Self {
        Bill { items_by_tax: MultiMap::new() }
    }

    pub fn add_item(&mut self, amount: Amount, product: Product<'a>) {
        let item = BillItem {
            amount: amount,
            product: product,
        };

        self.items_by_tax.insert(product.tax, item);
    }

    pub fn sums_by_tax(&self) -> BTreeMap<Tax, Currency> {
        self.items_by_tax
            .iter_all()
            .map(|(tax, items)| {
                (*tax, items.iter()
                            .map(|i| i.product.price * i.amount)
                            //.sum() // TODO add Currency
                            .fold(Money::default(), |acc, x| acc + x)
                            )
            })
            .collect()
    }

    pub fn taxes_by_tax(&self) -> BTreeMap<Tax, Money> {
        self.items_by_tax
            .iter_all()
            .map(|(tax, items)| {
                (*tax, items.iter()
                            .map(|i| i.product.price * i.amount * *tax.as_ref())
                            //.sum()
                            .fold(Money::default(), |acc, x| acc + x)
                            )
            })
            .collect()
    }

    pub fn total(&self) -> Money {
        self.items_by_tax
            .iter_all()
            .map(|(tax, items)| {
                items.iter()
                    .map(|i| i.product.price * i.amount * (*tax.as_ref()+1.0))
                    //.sum()
                    .fold(Money::default(), |acc, x| acc + x)
            })
        .fold(Money::default(), |acc, x| acc + x)
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
