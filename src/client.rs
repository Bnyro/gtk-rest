use reqwest::{Client, Response};

pub struct Request {
    url: String,
    body: String,
    method: u32,
}

impl Request {
    pub fn new(url: String, body: String, method: u32) -> Self {
        return Self { url, body, method };
    }

    pub async fn execute(&self) -> Result<Response, reqwest::Error> {
        let client = Client::new();
        let url = self.url.as_str();
        let request = match self.method {
            0 => client.get(url),
            1 => client.post(url),
            2 => client.put(url),
            3 => client.patch(url),
            4 => client.delete(url),
            5 => client.head(url),
            _ => client.get(url),
        };
        let body = self.body.clone();
        request.body(body).send().await
    }
}
