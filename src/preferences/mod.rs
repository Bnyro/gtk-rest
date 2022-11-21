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
    pub headers: Vec<Header>,
    pub queries: Vec<Query>,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Query {
    pub key: String,
    pub value: String,
}
