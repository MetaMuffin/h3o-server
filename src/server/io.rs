
use tokio::net::TcpStream;
use crate::server::response::Response;
use crate::server::router::Route;
use crate::server::request::Request;
use tokio::net::TcpListener;
use tokio::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use crate::server::router::HandleResult::Handled;
use crate::server::router::HandleResult::NoMatch;
use crate::server::router::HandleResult::Next;

static CONN_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct Server {
    root_path: Route,
}

impl Server {
    pub fn new(root: Route) -> Server {
        Server {
            root_path: root
        }
    }

    pub async fn main_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server running");
        let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
        
        loop {
            let (socket, _addr) = listener.accept().await?;
            CONN_COUNTER.fetch_add(1, Ordering::SeqCst);
            //println!("Somebody connected");
            let fut = Server::handle_connection(&self,socket);
            fut.await;
            //println!("Somebody disconnected")
        }
        //println!("Server stopped");
    }
    
    pub async fn handle_connection(&self, mut socket: TcpStream) {
        let mut buf = [0; 1024];
        loop {
            let _n = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            let req_str = match std::str::from_utf8(&buf) {
                Ok(s) => s,
                Err(_e) => return eprintln!("Error converting request to utf-8")
            };
            let req_string = String::from(req_str).replace("\u{0}", "");
            let req = match Request::create(req_string) {
                Some(r) => r,
                None => {println!("Cant parse header"); continue}
            };

            //println!("{:?}",req);
            let mut res = self.respond(req);
            let res_string = res.serialize();
            //println!("{0}",res_string);

            //let f = format!("HTTP/1.0 200 OK\r\nContent-Length: 5\r\nConnection: keep-alive\r\n\r\n{0}----------\n", CONN_COUNTER.load(Ordering::SeqCst));
            let resp_buf = res_string.as_bytes();
            if let Err(e) = socket.write(resp_buf).await {
                println!("{:?}",e);
            }
        }
        
    }

    pub fn respond(&self, req: Request) -> Response {
        let mut res = Response::new();
        match self.root_path.respond(req.path.as_str(), &req, &mut res) {
            Handled => {
                //println!("Handled route");
            },
            _ => {
                res.send("404 Not found");
                res.set_code(404, String::from("Not found"));
                //println!("No Route found");
            }
        }
        return res;
    }
}

