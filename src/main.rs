use std::{io, net::TcpListener};
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(),io::Error>{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    run(listener)?.await
}
