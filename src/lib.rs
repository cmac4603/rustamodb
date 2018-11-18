//! # rustamodb
//!
//! `rustamodb` is inspired by [pynamodb](https://pynamodb.readthedocs.io/en/latest/) and is a simple
//! to use library for AWS DynamoDB.
//! 
//! # Examples
//! 
//! ```
//! use rustamodb;
//! 
//! fn main() {
//!     match rustamodb::scan(&"my_dynamodb_table") {
//!         Some(ref items_list) => {
//!            for item in items_list {
//!                 print_ln!("{:?}", item);
//!            }
//!         },
//!         None => (),
//!     }
//! }
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
