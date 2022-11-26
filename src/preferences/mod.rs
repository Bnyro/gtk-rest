pub(crate) mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Preferences {
    pub workspaces: Vec<Workspace>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Workspace {
    pub name: String,
    pub requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Request {
    pub name: String,
    pub target_url: String,
    pub method: u32,
    pub headers: Vec<KeyValuePair>,
    pub queries: Vec<KeyValuePair>,
    pub body: String,
    pub response: String,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
