use std::net::TcpListener;
use zero2prod::run;

// this macro generates boiler plate to wrap async around our synchronous main
// since rust main can't be synchronous
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call.await on the server
    run(listener)?.await
}
