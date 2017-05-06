# 💸 Bill

[![Travis](https://img.shields.io/travis/hoodie/bill-rs.svg)](https://travis-ci.org/hoodie/bill-rs/)
[![license](https://img.shields.io/crates/l/bill.svg)](https://crates.io/crates/bill/)
[![Crates.io](https://img.shields.io/crates/d/bill.svg)](https://crates.io/crates/bill)
[![version](https://img.shields.io/crates/v/bill.svg)](https://crates.io/crates/bill/)
[![documentation](https://docs.rs/bill/badge.svg)](https://docs.rs/bill/)

Tiny little billing library.


## Example

```bash
cargo run --example catalogue
```

```
offer:
  0%
*   2x Service         8,00€ 16,00€
  0.19%
  *   8x Water           0,61€ 4,88€
  *   4x AppleJuice      1,64€ 6,56€
  *   4x OrangenJuice    1,86€ 7,44€
  *   40x Sandwich        3,40€ 136,00€
---------------------------------------
       170,88€
     + 29,43€ (tax)
   net 200,31€

...
```

### With serialization

```bash
cargo run --example catalogue --features serialization
```

```json
{"items_by_tax":{
  "0": {
    "items":[
      {"amount":2.0,"product":{"name":"Service","price":8.0,"tax":"0"}}
    ]
  },
  "0.19":{
    "items": [
      {"amount":8.0,"product":{"name":"Water","price":0.61,"tax":"0.19"}},
      {"amount":4.0,"product":{"name":"AppleJuice","price":1.64,"tax":"0.19"}},
      {"amount":4.0,"product":{"name":"OrangenJuice","price":1.86,"tax":"0.19"}},{"amount":40.0,"product":{"name":"Sandwich","price":3.4,"tax":"0.19"}}
    ]
   }
}}
```
