use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration"); //从文件中获取配置
                                                                                    //获取数据库连接
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    //监听地址
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address);
    run(listener.unwrap(), connection_pool)?.await
}
