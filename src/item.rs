#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Money, Amount, BillProduct};

/// Maps a `BillProduct` to an amount.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BillItem<P> {
    pub amount: Amount,
    pub product: P,
}

impl<P:BillProduct> BillItem<P> {
    /// `price * amount`
    pub fn gross(&self) -> Money {
        self.product.price() * self.amount
    }

    /// `price * tax * amount`, tax being less than 1.0
    pub fn tax(&self) -> Money {
        self.product.price() * **self.product.tax() * self.amount
    }

    /// `gross + tax`
    pub fn net(&self) -> Money {
        self.gross() + self.tax()
    }
}
