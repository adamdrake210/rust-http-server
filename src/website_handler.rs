use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::{fs};

pub struct WebSiteHandler {
  public_path: String,
}

// Constructor
impl WebSiteHandler {
  pub fn new(public_path: String) -> Self {
    Self {
      public_path,
    }
  }

  fn read_file(&self, file_path: &str) -> Option<String> {
    let path = format!("{}/{}", self.public_path, file_path);

    match fs::canonicalize(path) {
      Ok(path) => {
        if path.starts_with(&self.public_path) {
          fs::read_to_string(path).ok()
        } else {
          println!("Directory Traversal Attack Attempted: {}", file_path);
          None
        }
      }
      Err(_) => None
    }
  }
}

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok200, self.read_file("index.html")),
        "/hello" => Response::new(StatusCode::Ok200, Some("<h1>Hello</h1>".to_string())),
        path => match self.read_file(path) {
          Some(contents) => Response::new(StatusCode::Ok200, Some(contents)),
          None => Response::new(StatusCode::NotFound404, None),
        }
      },
      _ => Response::new(StatusCode::NotFound404, None),
    }
    
  }
}