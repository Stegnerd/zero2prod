use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// this macro generates boiler plate to wrap async around our synchronous main
// since rust main can't be synchronous
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // we use pool instead of connection because we need a mutable executor (guarantees that we are
    // unique and have access) Connection has that  but wed:Data<Connection> does not
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call.await on the server
    run(listener, connection_pool)?.await?;
    Ok(())
}
