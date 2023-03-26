# 💸 Bill

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/bill-rs/ci.yml?branch=main)](https://github.com/hoodie/bill-rs/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/bill)](https://crates.io/crates/bill)
[![contributors](https://img.shields.io/github/contributors/hoodie/bill-rs)](https://github.com/hoodie/bill-rs/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2023)

[![version](https://img.shields.io/crates/v/bill)](https://crates.io/crates/bill/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/bill/)
[![license](https://img.shields.io/crates/l/bill.svg?style=flat)](https://crates.io/crates/bill/)


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
  *   4x OrangeJuice    1,86€ 7,44€
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
      {"amount":4.0,"product":{"name":"OrangeJuice","price":1.86,"tax":"0.19"}},{"amount":40.0,"product":{"name":"Sandwich","price":3.4,"tax":"0.19"}}
    ]
   }
}}
```
