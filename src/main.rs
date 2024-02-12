use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, Robbert-Jan" }));

    // run our app with hyper, listening on localhost
    let listener = tokio::net::TcpListener::bind("localhost:3333")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
