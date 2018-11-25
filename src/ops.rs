use std::collections::HashMap;
use std::env;
use std::sync::{Mutex, Arc};

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient,
    DeleteItemInput,
    GetItemInput,
    ScanInput,
    PutItemInput,
};

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

pub type RustamoDbScanOutput = Vec<AttributeMap>;
pub type RustamoDbGetOutput = AttributeMap;
pub type RustamoDbAddOutput = AttributeMap;
pub type RustamoDbDelOutput = AttributeMap;
pub type RustamoDbOutput = AttributeMap;

pub type RustamoDbError = String;

pub struct GetItem { pub key: AttributeMap }

pub struct AddItem { pub item: AttributeMap }

pub struct DelItem { pub key: AttributeMap }

lazy_static! {
    static ref client: Arc<Mutex<DynamoDbClient>> = {
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

pub fn scan(table_name: TableName) -> Result<RustamoDbScanOutput, RustamoDbError> {
    // TODO: enable pagination of results or think of a way to handle this
    let scan_input: ScanInput = ScanInput {
        table_name: table_name.to_string(),
        ..Default::default()
    };
    match client.lock().unwrap().scan(&scan_input).sync() {
        Ok(scan_output) => {
            match scan_output.items {
                Some(items) => Ok(items),
                None => Err(format!("no items in db")),
            }
        },
        Err(_) => Err(format!("no db found")),
    }
}

pub fn get_item(table_name: TableName, attrs: GetItem) -> Result<RustamoDbGetOutput, RustamoDbError> {
    let get_item_input: GetItemInput = GetItemInput {
        table_name: table_name.to_string(),
        key: attrs.key,
        ..Default::default()
    };
    match client.lock().unwrap().get_item(&get_item_input).sync() {
        Ok(get_output) => {
            match get_output.item {
                Some(item) => Ok(item),
                None => Err(format!("item not found in dynamo_db")),
            }
        },
        Err(_) => Err(format!("unknown error")),
    }
}

trait Attributes {
    fn collect(self) -> Result<RustamoDbOutput, RustamoDbError>;
}

impl Attributes for Option<AttributeMap> {
    fn collect(self) -> Result<RustamoDbOutput, RustamoDbError> {
        match self {
            Some(item) => Ok(item),
            None => Err(format!("no attributes found")),
        }
    }
}


pub fn add_item(table_name: &str, attrs: AddItem) -> Result<RustamoDbAddOutput, RustamoDbError> {
    // TODO: the following will be a rustamodb model type
    let add_item_input: PutItemInput = PutItemInput {
        table_name: table_name.to_string(),
        item: attrs.item,
        ..Default::default()
    };
    match client.lock().unwrap().put_item(&add_item_input).sync() {
        Ok(add_item_output) => Attributes::collect(add_item_output.attributes),
        Err(error) => Err(error.to_string()),
    }
}

pub fn del_item(table_name: &str, attrs: DelItem) -> Result<RustamoDbDelOutput, RustamoDbError> {
    // TODO: the following will be a rustamodb model type
    let del_item_input: DeleteItemInput = DeleteItemInput {
        table_name: table_name.to_string(),
        key: attrs.key,
        ..Default::default()
    };
    match client.lock().unwrap().delete_item(&del_item_input).sync() {
        Ok(del_item_output) => Attributes::collect(del_item_output.attributes),
        Err(error) => Err(error.to_string()),
    }
}
