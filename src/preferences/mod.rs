use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Preferences {
    pub workspaces: Vec<Workspace>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Workspace {
    pub name: String,
    pub requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Request {
    pub name: String,
    pub target_url: String,
    pub method: String,
    pub headers: Vec<KeyValuePair>,
    pub queries: Vec<KeyValuePair>,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
