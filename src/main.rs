mod render;
mod routes;
mod sections;

use std::env;
use tiny_http::Server;
use routes::handle_request;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut port: u16 = 8080;

  let mut i = 1;
  while i < args.len() {
    match args[i].as_str() {
      "-p" | "--port" => {
        if i+1 < args.len() {
          match args[i+1].parse::<u16>() {
            Ok(num) => port = num,
            Err(_) => {
              eprintln!("Error: Invalid port number");
              return;
            }
          }
          i += 1;
        } else {
          eprint!("Unspecified port number");
          return;
        }
      },
      _ => {
        eprintln!("Unknown arguments: {}", args[i]);
      }
    }
    i += 1;
  }

  let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
  println!("Server is running on http://localhost:{}", port);

  for request in server.incoming_requests() {
    let response = handle_request(&request);
    request.respond(response).unwrap();
  }
}
