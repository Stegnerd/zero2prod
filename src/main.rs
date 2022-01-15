use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// this macro generates boiler plate to wrap async around our synchronous main
// since rust main can't be synchronous
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call.await on the server
    run(listener)?.await
}
