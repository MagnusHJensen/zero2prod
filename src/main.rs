use std::net::TcpListener;

use zero2prod::configuration::Settings;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = Settings::new().expect("Failed to read configuration");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Unable to bind to port 8000");
    run(listener)?.await
}
