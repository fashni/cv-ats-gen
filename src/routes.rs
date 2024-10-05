use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::LazyLock;
use tiny_http::{Header, Request, Response};
use urlencoding::decode;

use crate::render::render_cv;
use crate::sections::CV;

static HTML_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/html; charset=UTF-8").unwrap());
static CSS_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/css").unwrap());

pub fn handle_request(request: &Request) -> Response<Cursor<Vec<u8>>> {
  fn err_response(status_code: i32, message: &str) -> Response<Cursor<Vec<u8>>> {
    Response::from_string(message).with_status_code(status_code)
  }

  let url = request.url();
  match url {
    "/" => err_response(404, "404 Not Found"),

    "/static/style.css" => {
      if let Ok(content) = fs::read(&url[1..]) {
        Response::from_data(content).with_header(CSS_HEADER.clone())
      } else {
        err_response(404, "404 Not Found")
      }
    },

    _ => {
      let url_path = match decode(url.trim_start_matches('/')) {
        Ok(path) => path.to_string(),
        Err(_) => {
          return err_response(400, "400 Bad Request: Invalid URL encoding.");
        }
      };

      let base_name = if url_path.ends_with(".yaml") || url_path.ends_with(".yml") {
        url_path.trim_end_matches(".yaml").trim_end_matches(".yml")
      } else {
        &url_path
      };

      if let Some(file_path) = locate_file(base_name) {
        match CV::from_yaml(&file_path) {
          Ok(Some(cv)) => Response::from_string(render_cv(&cv).into_string()).with_header(HTML_HEADER.clone()),
          Ok(None) => err_response(404, "404 Not Found"),
          Err(_) => err_response(500, "500 Internal Server Error: Failed to parse YAML")
        }
      } else {
        err_response(404, "404 Not Found")
      }
    }
  }
}

fn locate_file(base_name: &str) -> Option<String> {
  let no_ext = format!("cv/{}", base_name);
  let yaml_file = format!("cv/{}.yaml", base_name);
  let yml_file = format!("cv/{}.yml", base_name);

  if Path::new(&no_ext).exists() {
    Some(no_ext)
  } else if Path::new(&yaml_file).exists() {
    Some(yaml_file)
  } else if Path::new(&yml_file).exists() {
    Some(yml_file)
  } else {
    None
  }
}
