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
    let port_key = "PORT";
    let default_port = 3000;
    let port = match std::env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "the port number \"{}\" is invalid. default port will be used.",
                    val
                );
                default_port
            }
        },
        Err(_) => {
            println!(
                "\"{}\" is not defined in environment variables. default port will be used.",
                port_key
            );
            default_port
        }
    };

    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/logout", get(logout));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}


