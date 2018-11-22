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
    AttributeMap,
    scan, Scan, ScanErr, RustamoDbScanOutput,
    get_item, Get, GetItem, GetErr, RustamoDbGetOutput,
    add_item, Add, AddItem, AddErr, RustamoDbAddOutput,
    del_item, Del, DelItem, DelErr,
    RustamoDbError,
};
