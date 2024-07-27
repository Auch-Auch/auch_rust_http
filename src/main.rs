use server::Server;
use std::env;
use static_handler::StaticHandler;
use crate::server::Router;

mod http;
mod server;
mod static_handler;

#[tokio::main]
async fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/public";
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let mut router = Router::new();
    router.set_static_handler(public_path);
    let server = Server::new(format!("127.0.0.1:{}", port));
    server.run(router).await;
}
