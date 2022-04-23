use server::Server;
use http::Request;
use http::Method;

mod http;
mod server; // like copy paste (include in C)

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}