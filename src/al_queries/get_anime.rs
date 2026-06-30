use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{al_queries::graphql_queries::GET_ANIME_QUERY, types::Anime};


#[derive(Debug, Deserialize)]
struct GraphQLResponse {
    data: MediaData,
}

#[derive(Debug, Deserialize)]
struct MediaData {
    #[serde(rename = "Media")]
    media: Option<Anime>,
}

pub async fn get_anime(id: i32) -> Result<Anime, Box<dyn std::error::Error + Send + Sync>>  {
    let client = Client::new();

    let body = json!({
        "query": GET_ANIME_QUERY,
        "variables": {
            "mediaId": id
        }
    });

    let response = client
        .post("https://graphql.anilist.co")
        .json(&body)
        .send()
        .await?;

    let text = response.text().await?;
    let data: GraphQLResponse = serde_json::from_str(&text)?;
    
    match data.data.media {
        Some(anime) => Ok(anime),
        None => Err("Anime not found (Media was null)".into()),
    }
}