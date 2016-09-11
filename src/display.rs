//! Implementation of `std::fmt::Display`
use std::fmt;
use ::{Product, BillItem, Bill};

impl<'a> fmt::Display for Product<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name} á {price}", name = self.name, price = self.price)
    }
}


impl<'a> fmt::Display for BillItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{a}x    {p} {s}",
               a = self.amount,
               p = self.product,
               s = self.sum())
    }
}


impl<'a> fmt::Display for Bill<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (tax, items) in self.items_by_tax.iter_all() {
            try!(write!(f, "{}%\n", tax.as_ref() * 100.));
            for item in items {
                try!(write!(f, "{:15} {} {} {:>9}\n", item.product.name, item.amount, item.product.price, item.amount * item.product.price, ));
            }
        }
        Ok(())
    }
}
