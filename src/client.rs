use reqwest::blocking::Response;

pub struct Request {
    url: String,
    method: u32,
}

impl Request {
    pub fn new(url: String, method: u32) -> Self {
        return Self { url, method };
    }

    pub fn execute(&self) -> Result<Response, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = self.url.as_str();
        let resp = match self.method {
            0 => client.get(url),
            1 => client.post(url),
            2 => client.put(url),
            3 => client.patch(url),
            4 => client.delete(url),
            5 => client.head(url),
            _ => client.get(url),
        };
        resp.send()
    }
}
