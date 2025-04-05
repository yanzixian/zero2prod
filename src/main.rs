use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration=get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind("127.0.0.1:8000");
    run(listener.unwrap())?.await
}
