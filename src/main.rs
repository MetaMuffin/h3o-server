
use tokio::net::TcpListener;
use tokio::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
//use std::str::from_utf8;

static CONN_COUNTER: AtomicI32 = AtomicI32::new(0);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        CONN_COUNTER.fetch_add(1, Ordering::SeqCst);
        tokio::spawn(async move {
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
                let f = format!("HTTP/1.0 200 OK\r\nContent-Length: 5\r\nConnection: keep-alive\r\n\r\n{0}----------\n", CONN_COUNTER.load(Ordering::SeqCst));
                let resp_buf = f.as_bytes();
                if let Err(e) = socket.write(resp_buf).await {
                    println!("{:?}",e)
                }
            }
        });
    }
}