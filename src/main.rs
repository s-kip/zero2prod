use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configurations::get_configurations;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configurations().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
