//! # rustamodb
//!
//! `rustamodb` is inspired by [pynamodb](https://pynamodb.readthedocs.io/en/latest/) and is a simple
//! to use library for AWS DynamoDB.
//! 
//! # Examples
//! 
//! ```
//! fn main() {
//!     match rustamodb::scan(&"my_dynamodb_table") {
//!         Ok(scan_output) => {
//!             for item in scan_output {
//!                 println!("{:?}", item);
//!             }
//!         },
//!         Err(_) => (),
//!     }
//! }
//! ```
#[macro_use]
extern crate lazy_static;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

mod ops;

pub use self::ops::{
    AttributeMap, Attributes,
    scan, Scan, ScanErr,
    get_item, Get, GetItem, GetErr,
    add_item, Add, AddItem, AddErr,
    del_item, Del, DelItem, DelErr,
};
