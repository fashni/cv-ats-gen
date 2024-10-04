use std::fs;
use std::sync::LazyLock;
use tiny_http::{Header, Request, Response};

use crate::render::render_cv;
use crate::sections::CV;

static HTML_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/html; charset=UTF-8").unwrap());
static CSS_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/css").unwrap());

pub fn handle_request(request: Request) {
  let url = request.url().to_string();
  if url == "/" {
    not_found(request);
  } else if url == "/static/style.css" {
    if let Ok(content) = fs::read(&url[1..]) {
      let response = Response::from_data(content).with_header(CSS_HEADER.clone());
      request.respond(response).unwrap();
    } else {
      not_found(request);
    }
  } else {
    let path = format!("cv{}", url);
    if let Some(cv) = CV::from_yaml(path.as_str()) {
      let response = Response::from_string(render_cv(&cv).into_string()).with_header(HTML_HEADER.clone());
      request.respond(response).unwrap();
    } else {
      not_found(request);
    }
  }
}

fn not_found(request: Request) {
  let response = Response::from_string("404 Not Found").with_status_code(404);
  request.respond(response).unwrap();
}
