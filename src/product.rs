#[cfg(feature = "serde")]
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::{Money, Tax};

/// Describes one particular product.
/// Amount is handled by `BillItem`
///
/// You can write your own product, just implement `BillProduct`
#[derive(Clone, Copy, Debug)]
pub struct Product<'a> {
    pub name: &'a str,
    pub price: Money,
    pub tax: Tax,
}

#[cfg(feature = "serde")]
impl<'a> Serialize for Product<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Product", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("price", &self.price.as_float())?;
        s.serialize_field("tax", &self.tax)?;
        s.end()
    }
}

impl<'a> Product<'a> {
    /// Instantiates a new Product.
    pub fn new(name: &'a str, price: Money, tax: f64) -> Self {
        Product {
            name,
            price,
            tax: Tax::new(tax),
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

impl BillProduct for Product<'_> {
    fn price(&self) -> Money {
        self.price
    }
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn tax(&self) -> Tax {
        self.tax
    }
}
