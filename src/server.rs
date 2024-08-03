use std::convert::TryFrom;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{Result as IoResult, AsyncReadExt, AsyncWriteExt};
use crate::http::{Request, Response, StatusCode, Method};
use crate::StaticHandler;
use async_trait::async_trait;

pub struct Router {
    routes: HashMap<String, Box<dyn Handler>>,
    routes_methods: HashMap<String, Method>,
    static_handler: Option<StaticHandler>,
}


impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new(), routes_methods: HashMap::new(), static_handler: None }
    }

    pub fn set_static_handler(&mut self, public_path: String) {
        self.static_handler = Some(StaticHandler::new(public_path));
    }

    pub fn add_route(&mut self, route: &str, method: Method, handler: Box<dyn Handler>) {
        self.routes.insert(route.to_string(), handler);
        self.routes_methods.insert(route.to_string(), method);
    }

    pub fn remove_route(&mut self, route: &str) {
        self.routes.remove(route);
    }

    pub async fn handle_request(&self, request: &Request<'_>) -> Response {
        match self.routes.get(request.path()) {
            Some(handler) => match self.routes_methods.get(request.path()) {
                Some(method) => if method == request.method() { 
                    handler.handle_request(request).await 
                } else { 
                    Response::new(StatusCode::MethodNotAllowed, None) 
                }
                None => Response::new(StatusCode::BadRequest, None),
            } 
            None => {
                if let Some(handler) = &self.static_handler {
                    handler.handle_request(request).await
                } else {
                    Response::new(StatusCode::NotFound, None)
                }
            },
        }
    }
}


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

    pub async fn run(self, router: Router) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).await.unwrap();
        let router = Arc::new(router);

        loop {
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    let router = Arc::clone(&router);
                    tokio::spawn(async move {

                        let mut buffer = Vec::new();

                        match read_stream(&mut stream, &mut buffer).await {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                let response: Response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        println!("{:?}", request);
                                        router.handle_request(&request).await
                                    }
                                    Err(e) => Response::new(StatusCode::BadRequest, Some(e.to_string())),
                                };

                                if let Err(e) = response.send(&mut stream).await {
                                    println!("Failed to send response: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}

async fn read_stream(stream: &mut tokio::net::TcpStream, buffer: &mut Vec<u8>) -> IoResult<()> {
    let mut temp_buffer = [0; 1024]; // Temporary buffer for reading chunks

    loop {
        let n = stream.read(&mut temp_buffer).await?;
        if n == 0 {
            break;
        }

        buffer.extend_from_slice(&temp_buffer[..n]);

        if is_end_of_request(&buffer) {
            break;
        }
    }

    Ok(())
}

fn is_end_of_request(buffer: &[u8]) -> bool {
    buffer.windows(4).any(|window| window == b"\r\n\r\n")
}
