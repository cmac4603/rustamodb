# rustamodb
[![Build Status](https://travis-ci.org/cmac4603/rustamodb.svg?branch=master)](https://travis-ci.org/cmac4603/rustamodb)
[![Crates.io](https://img.shields.io/crates/v/rustamodb.svg)](https://crates.io/crates/rustamo)

`rustamodb` is inspired by [pynamodb](https://pynamodb.readthedocs.io/en/latest/) and is a simple to use library for AWS DynamoDB.

_Currently **very** early stages WIP._

## Examples

```rust
extern crate rustamodb;

fn main() {
    match rustamodb::scan(&"my_dynamodb_table") {
        Ok(db_scan) => {
            match db_scan.items {
                Some(ref items_list) => {
                    for item in items_list {
                      println!("{:?}", item);
                    };
                },
                None => println!("no items in db!"),
            };
        },
        Err(_) => (),
    }
}
```
