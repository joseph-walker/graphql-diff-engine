use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Value;

mod arg_product;

#[derive(Deserialize, Debug)]
pub struct Args(pub HashMap<String, HashMap<String, Value>>);

#[derive(Debug, Serialize, PartialEq)]
pub struct QueryArgs(pub HashMap<String, Value>);

#[derive(Deserialize, Debug)]
pub struct Config {
    pub query_path: String,
    pub headers: HashMap<String, String>,
    pub args: Args,
}

pub use crate::arg_product::*;
