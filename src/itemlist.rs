use super::{BillItem, BillProduct, Money};
use std::ops::Deref;

/// A list of `BillItem`s, implements summing methods.
#[derive(Debug)]
pub struct ItemList<P> {
    items: Vec<BillItem<P>>
}

impl<P:BillProduct> ItemList<P> {
    pub fn from_vec(list:Vec<BillItem<P>>) -> Self{
        ItemList{ items: list }
    }

    pub fn new() -> Self{
        ItemList{ items: Vec::new()}
    }

    pub fn gross_sum(&self) -> Money {
        self.items.iter()
            .map(|i|i.cost())
            .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn tax_sum(&self) -> Money{
        self.items.iter()
            .map(|i|i.tax())
            .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn net_sum(&self) -> Money{
        self.items.iter()
            .map(|i|i.net())
            .fold(Money::default(), |acc, x| acc + x)
    }

    pub fn push(&mut self, item:BillItem<P>){
        self.items.push(item)
    }

}


impl<P: BillProduct> Deref for ItemList<P> {
    type Target = [BillItem<P>];
    fn deref(&self) -> &[BillItem<P>]{
        self.items.deref()
    }
}
