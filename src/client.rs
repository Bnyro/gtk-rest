use reqwest::blocking::{Client, Response};

use crate::preferences::KeyValuePair;

pub struct Request {
    url: String,
    body: String,
    method: u32,
    headers: Vec<KeyValuePair>,
    queries: Vec<KeyValuePair>,
}

impl Request {
    pub fn new(
        url: String,
        body: String,
        method: u32,
        headers: Vec<KeyValuePair>,
        queries: Vec<KeyValuePair>,
    ) -> Self {
        Self {
            url,
            body,
            method,
            headers,
            queries,
        }
    }

    pub async fn execute(&self) -> Result<Response, reqwest::Error> {
        let client = Client::new();
        let url = self.url.as_str();
        let mut request = match self.method {
            0 => client.get(url),
            1 => client.post(url),
            2 => client.put(url),
            3 => client.patch(url),
            4 => client.delete(url),
            5 => client.head(url),
            _ => client.get(url),
        };

        // insert all headers
        for index in 0..self.headers.len() {
            let pair = self.headers[index].clone();
            if pair.key.trim().is_empty() || pair.value.trim().is_empty() {
                continue;
            }
            request = request.header(pair.key.as_str(), pair.value.as_str());
        }

        // insert all queries
        for index in 0..self.queries.len() {
            let pair = self.queries[index].clone();
            if pair.key.trim().is_empty() || pair.value.trim().is_empty() {
                continue;
            }
            request = request.query(&[(pair.key, pair.value)]);
        }

        request = request.body(self.body.clone());
        request.send()
    }
}
