mod routes;

#[tokio::main]
async fn main() {
    let app: axum::Router = routes::router();
    let address = "0.0.0.0:3000"; // TODO -> place this inside a config/env file

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    println!("Listening on address: {}", address);

    axum::serve(listener, app)
        .await
        .expect("server failed");

}
