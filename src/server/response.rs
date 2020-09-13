use std::collections::HashMap;


pub struct Response {
    headers: HashMap<String,String>,
    content: String
}

impl Response {
    pub fn serialize(mut self) -> String {
        let o = String::from("");
        self.headers.insert(String::from("Content-Length"), self.content.len().to_string());
        return o;
    }

    pub fn new() -> Response {
        Response {
            headers: HashMap::new(),
            content: String::new()
        }
    }

    pub fn send(&mut self,s: &str) {
        self.content += s;
    }
}