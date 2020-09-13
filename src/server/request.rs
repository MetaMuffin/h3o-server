

use crate::server::response::Response;

type HTTPHeaders = std::collections::HashMap<String,String>;

struct HTTPHeaderParseError {}


pub struct Request {
    pub headers: std::collections::HashMap<String,String>,
    pub action: String,
    pub path: String,
    pub http_version: String,
    pub body: String,

}

impl Request {
    pub fn create(req_s: String) -> Option<Request> {
        
        let headers_start = req_s.find("\r\n")?;
        let headers_end_o = req_s.find("\r\n\r\n");

        let (action_section,header_body_section) = req_s.split_at(headers_start);
        let (header_section,body_section) = match headers_end_o {
            Some(headers_end) => header_body_section.split_at(headers_end),
            None => (header_body_section,"")
        };
        
        let headers = match Request::parse_headers(header_section) {
            Ok(m) => m,
            Err(e) => {
                println!("Error headers cant be parsed");
                return None;
            }
        };
        
        let (action,path_and_version) = match action_section.find(" ") {
            Some(path_start) => action_section.split_at(path_start),
            None => {
                println!("Action section is invalid");
                return None;
            }
        };

        let (path,http_version) = match path_and_version.find(" ") {
            Some(version_start) => action_section.split_at(version_start),
            None => {
                println!("Action section is invalid");
                return None;
            }
        };

        return Some(Request {
            headers: headers,
            path: String::from(path),
            action: String::from(action),
            http_version: String::from(http_version),
            body: String::from(body_section)
        })
    }

    fn parse_headers(s: &str) -> Result<HTTPHeaders, HTTPHeaderParseError> {
        let mut hs = std::collections::HashMap::new();
        let lines = s.split("\r\n");

        for l in lines {
            if let Some(col_index) = l.find(":") {
                let (key,value) = l.split_at(col_index);
                hs.insert(key.to_string(), value.to_string())
            } else { continue };
        }

        return Ok(hs)
    }

}
