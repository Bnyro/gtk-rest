
use ureq::Response;

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

    pub fn execute(&self) -> Result<Response, ureq::Error> {
        let url = self.url.as_str();
        let mut request = match self.method {
            0 => ureq::get(url),
            1 => ureq::post(url),
            2 => ureq::put(url),
            3 => ureq::patch(url),
            4 => ureq::delete(url),
            5 => ureq::head(url),
            _ => ureq::get(url),
        }.set("User-Agent", "gtk-rest");

        if !self.body.trim().is_empty() {
            request = request.set("Content-Type", "application/json");
        }

        // insert all headers
        for index in 0..self.headers.len() {
            let pair = self.headers[index].clone();
            if pair.key.trim().is_empty() || pair.value.trim().is_empty() {
                continue;
            }
            request = request.set(pair.key.as_str(), pair.value.as_str());
        }

        // insert all queries
        for index in 0..self.queries.len() {
            let pair = self.queries[index].clone();
            if pair.key.trim().is_empty() || pair.value.trim().is_empty() {
                continue;
            }
            request = request.query(pair.key.as_str(), pair.value.as_str());
        }

        request.send_string(&self.body)
    }
}
