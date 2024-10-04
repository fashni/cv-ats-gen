mod render;
mod routes;
mod sections;

use tiny_http::Server;
use routes::handle_request;

fn main() {
  let server = Server::http("0.0.0.0:8080").unwrap();
  println!("Server is running on http://localhost:8080");

  for request in server.incoming_requests() {
    handle_request(request);
  }
}
