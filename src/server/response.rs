use std::collections::HashMap;


pub struct Response {
    headers: String,
    pub content: String,
    pub code: i32,
    pub code_message: String,
}

impl Response {
    pub fn serialize(&mut self) -> String {
        self.insertHeader("Content-Length",&self.content.len().to_string()[..]);
        return format!("{0} {1} {2}\r\n{3}\r\n{4}\r\n",
            "HTTP/1.1",
            self.code,
            self.code_message,
            self.headers,
            self.content
        );
    }

    pub fn insertHeader(&mut self ,key: &str, value: &str) {
        self.headers += &format!("{0}: {1}\r\n",key,value)[..];
    }

    pub fn set_code(&mut self, code: i32, msg: String) {
        self.code = code;
        self.code_message = msg;
    }
    
    pub fn new() -> Response {
        Response {
            headers: String::new(),
            content: String::new(),
            code: 200,
            code_message: String::from("OK")
        }
    }

    pub fn send(&mut self,s: &str) {
        self.content += s;
    }
}