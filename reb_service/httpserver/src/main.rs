use crate::server::Server;

mod server;
mod router;
mod handler;

fn main() {
    let server = Server::new("localhost:3000");

    server.run();
}
