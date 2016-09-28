//! Implements `to_json()`.
use std::fmt;

use rustc_serialize::json::{ToJson, Json};

use ::{Product, BillItem, Bill, BillProduct};

impl ToJson for BillProduct {
    fn to_json(&self) -> Json{
        Json::Object(btreemap!{
            String::from("name")  => self.name().to_json(),
            String::from("price") => self.price().to_json(),
            String::from("tax")   => self.tax().to_json(),
        })
    }
}


impl<P:BillProduct + fmt::Display> ToJson for BillItem<P> {
    fn to_json(&self) -> Json{
        Json::Object(btreemap!{
            String::from("amount") => self.amount.to_json(),
            String::from("product") => self.product.to_json(),
            String::from("sum") => self.sum().to_json(),
        })
    }
}


impl<P:BillProduct> ToJson for Bill<P> {
    fn to_json(&self) -> Json{
        Json::Object(btreemap!{
        })
    }
}
