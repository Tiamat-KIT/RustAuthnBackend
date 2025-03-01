use axum::{
    extract::Query,
    response::Redirect
};

use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

const BOX_AUTH_URL: &str = "https://account.box.com/api/oauth2/authorize";
const BOX_TOKEN_URL: &str = "https://api.box.com/oauth2/token";
const BOX_USER_INFO_URL: &str = "https://api.box.com/2.0/users/me";

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String
}

pub async fn login() -> Redirect {
    dotenv().expect(".env file not found");
    let client_id = env::var("BOX_CLIENT_ID").expect("BOX_CLIENT_ID is not set");
    let redirect_uri = env::var("BOX_REDIRECT_URI").expect("BOX_REDIRECT_URI is not set");

    let auth_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}",
        BOX_AUTH_URL,client_id,urlencoding::encode(&redirect_uri)
    );

    Redirect::temporary(&auth_url)
}

pub async fn callback(
    Query(params): Query<AuthRequest>
) -> String {
    let client_id = env::var("BOX_CLIENT_ID").unwrap();
    let client_secret = env::var("BOX_CLIENT_SECRET").unwrap();
    let redirect_uri = env::var("BOX_REDIRECT_URI").unwrap();

    let client = Client::new();
    let token_response = client
        .post(BOX_TOKEN_URL)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let access_token = token_response["access_token"].as_str().unwrap().to_string();

    // Boxのユーザー情報を取得
    let user_info = client
        .get(BOX_USER_INFO_URL)
        .bearer_auth(&access_token)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    format!("Logged in as: {}", user_info["name"])
}

pub async fn logout() -> String {
    "Logged out".to_string()
}