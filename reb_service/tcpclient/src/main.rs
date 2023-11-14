use std::net::TcpStream;
fn main() {
    let _stream = TcpStream::connect("localhost:3000").unwrap();
    println!("Hello, world!");
}
