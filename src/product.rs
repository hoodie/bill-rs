use super::{Money, Tax};

use ordered_float::OrderedFloat;

fn tax_from_f64(float: f64) -> Tax {
    OrderedFloat(float)
}

#[derive(Clone, Copy, Debug)]
/// Describes one particular product.
/// Amount is handled by `BillItem`
///
/// You can write your own product, just implement `BillProduct`
pub struct Product<'a> {
    pub name: &'a str,
    pub price: Money,
    pub tax: Tax,
}

impl<'a> Product<'a> {

    /// Instantiates a new Product.
    pub fn new(name: &'a str, price: Money, tax: f64) -> Self {
        Product {
            name: name,
            price: price,
            tax: tax_from_f64(tax),
        }
    }
}

/// Describes one particular product.
/// Amount is handled by `BillItem`
pub trait BillProduct {
    /// Price in Money
    fn price(&self) -> Money;

    /// A [name](https://en.wikipedia.org/wiki/Name) is a term used for identification.
    fn name(&self) -> String;

    /// Tax
    ///
    ///
    /// > A [tax](https://en.wikipedia.org/wiki/Tax) (from the Latin [taxo](https://en.wiktionary.org/wiki/en:taxo#Latin)) is a financial charge or other levy imposed upon a taxpayer (an individual or legal entity) by a state or the functional equivalent of a state to fund various public expenditures.
    ///
    fn tax(&self) -> Tax;
}

impl<'a> BillProduct for Product<'a> {
    fn price(&self) -> Money {self.price}
    fn name(&self) -> String {self.name.to_owned()}
    fn tax(&self) -> Tax {self.tax}
}


