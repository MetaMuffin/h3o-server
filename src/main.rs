
use crate::server::io::Server;
use server::router::Route;

mod server;


#[tokio::main]
async fn main() {
    let root = Route::new(vec![],|_req,res| {
        res.send("Hallo");
        //println!("Index page requested");
        server::router::HandleResult::Handled
    });
    
    let server = Server::new(root);
    
    if let Err(e) =  server.main_loop().await {
        println!("{:?}",e)    
    }
    
    
}