use std::env;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient,
    DeleteItemInput,
    GetItemInput,
    ScanInput,
    PutItemInput,
};

lazy_static! {
    static ref ddb_conn: Arc<Mutex<DynamoDbClient>> = {
        if let Ok(endpoint) = env::var("DYNAMODB_ENPOINT") {
            Arc::new(Mutex::new(DynamoDbClient::simple(
                Region::Custom {
                    name: env::var("AWS_DEFAULT_REGION").unwrap_or(format!("us-east-1")),
                    endpoint: endpoint.to_string(),
                },
            )))
        } else {
            Arc::new(Mutex::new(DynamoDbClient::simple(Region::default())))
        }
    };
}

type TableName = &'static str;

pub type Scan = rusoto_dynamodb::ScanOutput;
pub type ScanErr = rusoto_dynamodb::ScanError;

pub type Get = rusoto_dynamodb::GetItemOutput;
pub type GetErr = rusoto_dynamodb::GetItemError;

pub type Add = rusoto_dynamodb::PutItemOutput;
pub type AddErr = rusoto_dynamodb::PutItemError;

pub type Del = rusoto_dynamodb::DeleteItemOutput;
pub type DelErr = rusoto_dynamodb::DeleteItemError;

pub type AttributeMap = HashMap<String, AttributeValue>;
pub type Attributes = Vec<AttributeMap>;

pub struct GetItem { pub key: AttributeMap }

pub struct AddItem { pub item: AttributeMap }

pub struct DelItem { pub key: AttributeMap }

pub fn scan(table_name: TableName) -> Result<Vec<AttributeMap>, String> {
    let scan_input: ScanInput = ScanInput {
        table_name: table_name.to_string(),
        ..Default::default()
    };
    match ddb_conn.lock().unwrap().scan(&scan_input).sync() {
        Ok(scan_output) => {
            match scan_output.items {
                Some(items) => Ok(items),
                None => Err(format!("no items in db")),
            }
        },
        Err(_) => Err(format!("no db found")),
    }
}

pub fn get_item(table_name: TableName, attrs: GetItem) -> Result<Get, GetErr> {
    let get_item_input: GetItemInput = GetItemInput {
        table_name: table_name.to_string(),
        key: attrs.key,
        ..Default::default()
    };
    ddb_conn.lock().unwrap().get_item(&get_item_input).sync()
}

pub fn add_item(table_name: &str, attrs: AddItem) -> Result<Add, AddErr> {
    let add_item_input: PutItemInput = PutItemInput {
        table_name: table_name.to_string(),
        item: attrs.item,
        ..Default::default()
    };
    ddb_conn.lock().unwrap().put_item(&add_item_input).sync()
}

pub fn del_item(table_name: &str, attrs: DelItem) -> Result<Del, DelErr> {
    let del_item_input: DeleteItemInput = DeleteItemInput {
        table_name: table_name.to_string(),
        key: attrs.key,
        ..Default::default()
    };
    ddb_conn.lock().unwrap().delete_item(&del_item_input).sync()
}
