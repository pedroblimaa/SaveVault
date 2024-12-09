use std::env;

use dotenv::dotenv;
use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::models::game::GameInfo;

#[derive(Deserialize, Serialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct GameResponse {
    id: i64,
    name: String,
    cover: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CoverResponse {
    id: i64,
    url: String,
}

const BASE_URL: &str = "https://api.igdb.com/v4";

pub async fn get_token() -> String {
    dotenv().ok();

    let client_id = env::var("IGDB_CLIENT_ID").unwrap();
    let client_secret = env::var("IGDB_SECRET").unwrap();

    let url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        client_id, client_secret
    );

    let client = reqwest::Client::new();
    let response = client.post(&url).send().await.unwrap();

    let token_response: TokenResponse = response.json().await.unwrap();

    token_response.access_token
}

pub async fn get_game_info(name: &str, token: &str) -> GameInfo {
    let game = get_game(name, token).await;
    let cover = get_cover(game.cover.unwrap(), token).await;
    let name = game.name.clone();

    GameInfo {
        name: Some(name),
        url: Some(cover),
    }
}

async fn get_game(name: &str, token: &str) -> GameResponse {
    let body = format!("fields cover,name; search \"{}\";", name);
    let url = format!("{}/games", BASE_URL);

    println!("Body: {}", body);

    let res = make_request(&url, &body, token).await;
    let result: Vec<GameResponse> = res.json().await.unwrap();
    // TODO - Change to get the first that has cover instead
    let game = result.first().unwrap().clone();

    game
}

async fn get_cover(cover: i64, token: &str) -> String {
    let body = format!("fields url; where id = {};", cover);
    let url = format!("{}/covers", BASE_URL);

    let res = make_request(&url, &body, token).await;
    let result: Vec<CoverResponse> = res.json().await.unwrap();
    let cover = result.first().unwrap();
    let url = cover.url.replace("//", "https://").replace("t_thumb", "t_original");

    url.to_string()
}

async fn make_request(url: &str, body: &str, token: &str) -> Response {
    dotenv().ok();
    let client = reqwest::Client::new();

    let client_id = env::var("IGDB_CLIENT_ID").unwrap();

    client
        .post(url)
        .header("Client-ID", client_id)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
}
