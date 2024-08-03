use super::http::{Request, Response, StatusCode};
use crate::server::Handler;
use tokio::fs;

pub struct StaticHandler {
    public_path: String,
}

impl StaticHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    async fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
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
impl Handler for StaticHandler {
    async fn handle_request(&self, request: &Request) -> Response {
        match request.path() {
           "/" => Response::new(StatusCode::Ok, self.read_file("index.html").await),
            path => match self.read_file(path).await {
                Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                None => Response::new(StatusCode::NotFound, None),
            },
        }
    }
}
