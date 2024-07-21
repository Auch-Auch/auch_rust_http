use super::http::{Request, Response, StatusCode, Method};
use crate::server::Handler;
use tokio::fs;
use tokio::time::{sleep, Duration};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    async fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        sleep(Duration::from_millis(200)).await;
        match fs::canonicalize(&path).await {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).await.ok()
                } else {
                    println!("Attempted read of file outside of public_path");
                    None
                }
            }
            Err(_) => None,
        }
    }
}

#[async_trait::async_trait]
impl Handler for WebsiteHandler {
    async fn handle_request(&self, request: &Request) -> Response {
        println!("Received a request: {}", request.path());
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html").await),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello World!".to_string())),
                path => match self.read_file(path).await {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}