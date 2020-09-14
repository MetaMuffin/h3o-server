


use super::helper;

type HTTPHeaders = std::collections::HashMap<String,String>;

struct HTTPHeaderParseError {}

#[derive(Debug)]
pub struct Request {
    pub headers: std::collections::HashMap<String,String>,
    pub action: String,
    pub path: String,
    pub http_version: String,
    pub body: String,

}

impl Request {
    pub fn create(req_s: String) -> Option<Request> {
        
        let headers_start = req_s.find("\r\n")?; // TODO
        let (action_section,header_body_section) = req_s.split_at(headers_start);

        if let Some((header_section,body_section)) = helper::split(header_body_section, "\r\n\r\n") {
            if let Ok(headers) = Request::parse_headers(header_section) {
                if let Some((action,path_and_version)) = helper::split(action_section, " ") {
                    if let Some((path,http_version)) = helper::split(path_and_version, " ") {
                        return Some(Request {
                            headers: headers,
                            path: String::from(path),
                            action: String::from(action),
                            http_version: String::from(http_version),
                            body: String::from(body_section)
                        });
                    } else { println!("could not split path and version"); None }
                } else { println!("could not split action and path"); None }
            } else { println!("could not parse headers"); None }
        } else { println!("could not split headers and body"); None }
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
