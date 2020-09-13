use crate::server::io::Server;
use server::router::Route;

mod server;



#[tokio::main]
async fn main() {
    let root = Route::new(vec![],|req,res| {
        

        server::router::HandleResult::Handled
    });
    let mut server = Server::new(root);
    server.mainLoop();
    

}