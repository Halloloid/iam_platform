use iam_platform::config::{db_config::connect_db, server_config};

#[tokio::main]
async fn main() {
    let pool = connect_db().await.expect("Failed To Connect DB");
    println!("Connected to DB");
    server_config::run_server(pool).await;
}
