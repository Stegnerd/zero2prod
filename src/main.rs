use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

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
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call.await on the server
    run(listener, connection_pool)?.await?;
    Ok(())
}
