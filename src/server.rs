use std::convert::TryFrom;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use crate::http::{StatusCode, Response, Request};
use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle_request(&self, request: &Request) -> Response;
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub async fn run(self, handler: impl Handler + 'static) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).await.unwrap();
        let handler = std::sync::Arc::new(handler);

        loop {
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    let handler = handler.clone();

                    tokio::spawn(async move {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer).await {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                let response: Response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        let result = handler.handle_request(&request).await;
                                        result
                                    }
                                    Err(e) => Response::new(StatusCode::BadRequest, Some(e.to_string())),
                                };
                                if let Err(e) = response.send(&mut stream).await {
                                    println!("Failed to send response: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        };
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}
