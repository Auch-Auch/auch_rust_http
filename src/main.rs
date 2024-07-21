use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

#[tokio::main]
async fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/public";
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path)).await;
}
