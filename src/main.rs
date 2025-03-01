mod auth;

use auth::{
    r#box::{login,logout,callback}
};

use axum::{
    routing::{get,post},
    http::StatusCode,
    Json,Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/logout", get(logout));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}


