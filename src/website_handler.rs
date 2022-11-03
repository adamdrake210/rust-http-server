use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok200, Some("<h1>Welcome</h1>".to_string())),
        "/hello" => Response::new(StatusCode::Ok200, Some("<h1>Hello</h1>".to_string())),
        _ => Response::new(StatusCode::NotFound404, None),
      },
      _ => Response::new(StatusCode::NotFound404, None),
    }
    
  }
}