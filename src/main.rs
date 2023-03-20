use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::Settings;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = Settings::new().expect("Failed to read configuration");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres connection pool.");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .expect("Unable to bind to port 8000");
    run(listener, connection_pool)?.await
}
