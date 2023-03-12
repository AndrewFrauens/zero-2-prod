use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    println!();
    println!("serving on: http://127.0.0.1:{port}");
    println!();

    run(listener)?.await
}
