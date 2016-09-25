//! Product and Invoicing primitives.
//!
//! There is no difference between Invoices and Offers, so here there are only `Bill`s.
//! If you need to have Offers and invoices use two different `Bill`s.

extern crate ordered_float;
extern crate claude;

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;
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

pub trait BillProduct{
    fn price(&self) -> Money;
    fn name(&self) -> String;
    fn tax(&self) -> Tax;
}

impl<'a> BillProduct for Product<'a>{
    fn price(&self) -> Money {self.price}
    fn name(&self) -> String {self.name.to_owned()}
    fn tax(&self) -> Tax {self.tax}
}

#[derive(Debug)]
pub struct BillItem<P> {
    pub amount: Amount,
    pub product: P,
}

impl<P:BillProduct> BillItem<P> {

    pub fn sum(&self) -> Money {
        self.product.price() * self.amount
    }

}

#[derive(Debug)]
pub struct Bill<P> {
    pub items_by_tax: BTreeMap<Tax, Vec<BillItem<P>>>,
}

impl<P:BillProduct> Bill<P> {
    pub fn new() -> Self {
        Bill { items_by_tax: BTreeMap::new() }
    }

    pub fn to_items_with_tax(&self) -> BTreeMap<Tax, &BillItem<P>> {
        self.as_items_with_tax().into_iter().collect()
    }

    pub fn as_items_with_tax(&self) -> Vec<(Tax, &BillItem<P>)> {
        let mut out = Vec::new();
        for (tax, items) in self.items_by_tax.iter().rev() {
            for item in items.iter(){
                out.push((tax.to_owned(), item));
            }
        }
        out
    }

    pub fn as_items(&self) -> Vec<&BillItem<P>> {
        self.as_items_with_tax().into_iter().map(|(_,item)|item).collect()
    }

    pub fn add(&mut self, item:BillItem<P>) {
        let tax = item.product.tax();
        self.items_by_tax.entry(tax).or_insert_with(Vec::new).push(item);
    }

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
            .map(|(tax, items)| {
                (*tax, items.iter()
                            .map(|i| i.product.price() * i.amount)
                            //.sum() // TODO add Currency
                            .fold(Money::default(), |acc, x| acc + x)
                            )
            })
            .collect()
    }

    pub fn taxes_by_tax(&self) -> BTreeMap<Tax, Money> {
        self.items_by_tax
            .iter()
            .map(|(tax, items)| {
                (*tax, items.iter()
                            .map(|i| i.product.price() * i.amount * *tax.as_ref())
                            //.sum()
                            .fold(Money::default(), |acc, x| acc + x)
                            )
            })
            .collect()
    }

    pub fn total_before_tax(&self) -> Money {
        self.items_by_tax
            .iter()
            .map(|(_, items)| {
                items.iter()
                    .map(|i| i.product.price() * i.amount)
                    //.sum()
                    .fold(Money::default(), |acc, x| acc + x)
            })
        .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn total(&self) -> Money {
        self.items_by_tax
            .iter()
            .map(|(tax, items)| {
                items.iter()
                    .map(|i| i.product.price() * i.amount * (*tax.as_ref()+1.0))
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
