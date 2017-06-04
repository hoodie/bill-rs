#![allow(dead_code,unused_variables)]
extern crate bill;
extern crate ordered_float;
#[cfg(feature="serialization")] extern crate serde_json;

use bill::*;

fn c(value: i64) -> Currency {
    Currency {
        symbol: Some('€'),
        .. Currency::from_value(value)
    }
}

fn print_items(items: &[BillItem<Product>]) {
    //println!("{:?}", items);
    for item in items {
        println!("   * {:3}x {:15} {:6} {:6}", item.amount, item.product.name, item.product.price.postfix(), item.gross().postfix());
    }
}

fn print(title: &str, bill: &Bill<Product>) {

    println!("{}:", title);
    for (tax,items) in &bill.items_by_tax {
        println!("  {}%", tax);
        print_items(items);
    }
    println!("---------------------------------------");
    println!("       {}", bill.gross_total().postfix());
    println!("     + {} (tax)", bill.tax_total().postfix());
    println!("   net {}", bill.net_total().postfix());
    println!();
}

fn main() {
    let coffee       = Product::new("Coffee", c(0250), 0.19);
    let tee          = Product::new("Tea", c(0175), 0.19);
    let water        = Product::new("Water", c(0061), 0.19);
    let applejuice   = Product::new("AppleJuice", c(0164), 0.19);
    let orangejuice  = Product::new("OrangenJuice", c(0186), 0.19);
    let bagel        = Product::new("Bagel", c(0219), 0.19);
    let sandwich     = Product::new("Sandwich", c(0340), 0.19);
    let pie          = Product::new("pie", c(0094), 0.19);
    let soup         = Product::new("Soup", c(0310), 0.19);
    let service      = Product::new("Service", c(0800), 0.00);


    let mut offer = Bill::new();
    offer.add_item(8., water);
    offer.add_item(4., applejuice);
    offer.add_item(4., orangejuice);
    offer.add_item(40., sandwich);
    offer.add_item(2., service);


    let mut invoice = Bill::new();
    invoice.add_item(2., water);
    invoice.add_item(0., applejuice);
    invoice.add_item(2., orangejuice);
    invoice.add_item(50., sandwich);
    invoice.add_item(2.5, service);

    let mut invoice2 = Bill::new();
    invoice2.add_item(99.0, Product::new("Red Ballons", c(99), 0.19));

    #[cfg(not(feature="serialization"))]
    {
        print("offer", &offer);
        print("invoice", &invoice);
        print("invoice2", &invoice2);
    }
    #[cfg(feature="serialization")]
    {
        println!("{}", serde_json::to_string(&offer).unwrap());
        println!("{}", serde_json::to_string(&invoice).unwrap());
        println!("{}", serde_json::to_string(&invoice2).unwrap());
    }
}
