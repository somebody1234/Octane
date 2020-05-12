#[macro_use]
extern crate lazy_static;
pub mod constants;
pub mod http;
pub mod query;
pub mod responder;
pub mod server;
pub mod time;

fn main() {
    server::Server::listen("0.0.0.0:3333");
}