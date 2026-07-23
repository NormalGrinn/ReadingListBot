use std::{env, sync::Arc};

use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use tokio::sync::Mutex;

use crate::database::{get_english_and_romaji_titles, get_resource_titles};

mod types;
mod database;
mod al_queries;
mod commands;
mod types_display;
mod helpers;

struct Data {
    pub db: Arc<Mutex<rusqlite::Connection>>,
    pub add_users: Vec<u64>,
    pub remove_users: Vec<u64>,
    pub anime_names: Arc<Mutex<Vec<String>>>,
    pub resource_titles: Arc<Mutex<Vec<String>>>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let conn = rusqlite::Connection::open("databases/resources.db").expect("Failed to open database");
    let add_users = env::var("ADD_USERS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|id| id.trim().parse().ok())
        .collect();
    let remove_users = env::var("REMOVE_USERS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|id| id.trim().parse().ok())
        .collect();
    let mut anime_names = get_english_and_romaji_titles(&conn).expect("Error fetching anime names");
    anime_names.sort();
    let mut resource_titles = get_resource_titles(&conn).expect("Error fetching resource titles");
    resource_titles.sort();
    let token = env::var("TOKEN")
        .expect("Missing `TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() 
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;
    let data = Data {
        db: Arc::new(Mutex::new(conn)),
        add_users,
        remove_users,
        anime_names: Arc::new(Mutex::new({
            let mut v = anime_names;
            v.sort();
            v.dedup();
            v
        })),
        resource_titles: Arc::new(Mutex::new({
            let mut v = resource_titles;
            v.sort();
            v
        })),
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::add_anime::add_anime(),
                commands::add_resource::add_resource(),
                commands::show_anime::show_anime(),
                commands::link_resource::link_resource(),
                commands::show_resource::show_resource(),
                commands::list_resources::list_resources(),
                commands::unlink_resource::unlink_resource(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}