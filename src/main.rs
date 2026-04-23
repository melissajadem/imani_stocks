mod claude;
mod routes;
mod yahoo;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Imani Stocks starting on http://localhost:3000");

    let app = Router::new().merge(routes::create_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
