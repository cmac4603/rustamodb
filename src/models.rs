//! # Using the rustomdb `model!` macro
//! 
//! ```
//! #[macro_use]
//! extern crate rustamodb;
//! 
//! use rustamodb::models::{DynamoDbAttribute, DynamoValue};
//! 
//! 
//! fn main() {
//!     model!(
//!         my_key_one: DynamoDbAttribute,
//!         my_key_two: DynamoDbAttribute
//!     );
//! 
//!     let my_model = DynamoDbItem {
//!         my_key_one: DynamoDbAttribute {
//!             value: DynamoValue::StringType(String::from("hello")),
//!         },
//!         my_key_two: DynamoDbAttribute {
//!             value: DynamoValue::Number(8),
//!         },
//!     };
//! }
//! ```
use std::collections::HashMap;
use rusoto_dynamodb::{AttributeValue};

pub enum DynamoValue {
    Binary(Vec<usize>),
    Bool(bool),
    BinarySet(Vec<Vec<usize>>),
    List(Vec<AttributeValue>),
    Map(HashMap<String, AttributeValue>),
    Number(usize),
    NumberSet(Vec<String>),
    Null(bool),
    StringType(String),
    StringSet(Vec<String>),
}

pub struct DynamoDbAttribute {
    pub value: DynamoValue,
}

#[macro_export]
macro_rules! model {
    ($($element: ident: $ty: ty),*) => {
        pub struct DynamoDbItem { $($element: $ty),* }
    }
}
