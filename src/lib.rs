//! # rustamodb
//!
//! `rustamodb` is inspired by [pynamodb](https://pynamodb.readthedocs.io/en/latest/) and is a simple
//! to use library for AWS DynamoDB.
//! 
//! # Examples
//! 
//! ```
//! fn main() {
//!    match rustamodb::scan(&"my_dynamodb_table") {
//!        Ok(db_scan) => {
//!            match db_scan.items {
//!                Some(ref items_list) => {
//!                    for item in items_list {
//!                        println!("{:?}", item);
//!                    };
//!                },
//!                None => println!("no items in db!"),
//!            };
//!        },
//!        Err(_) => (),
//!    }
//!}
//! ```
#[macro_use]
extern crate lazy_static;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

mod ops;

pub use self::ops::{
    scan, Scan, ScanErr,
    get_item, Get, GetItem, GetErr,
    add_item, Add, AddItem, AddErr,
    del_item, Del, DelItem, DelErr,
};
