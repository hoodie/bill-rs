use super::{Money, Amount, BillProduct};

/// Mapps a `BillProduct` to an amount.
#[derive(Debug)]
pub struct BillItem<P> {
    pub amount: Amount,
    pub product: P,
}

impl<P:BillProduct> BillItem<P> {
    /// price * amount
    pub fn cost(&self) -> Money {
        self.product.price() * self.amount
    }

    /// price * tax * amount
    pub fn tax(&self) -> Money {
        self.product.price() * *self.product.tax() * self.amount
    }

    /// cost + tax
    pub fn net(&self) -> Money {
        self.cost() + self.tax()
    }
}
