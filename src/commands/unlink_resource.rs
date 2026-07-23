use std::pin::Pin;

use poise::CreateReply;
use rusqlite::Result;
use serenity::futures::{self, Stream};

use crate::{Context, Error, database::{self, get_resource_by_id, get_resource_id_by_name}, helpers::{create_resource_show_embed, fuzzy_autocomplete, is_remove_authorized}, types::MediaType};

use futures::stream::{self};

async fn autocomplete_resource<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> Pin<Box<dyn Stream<Item = String> + Send + 'a>> {
    let anime_title = match ctx {
        poise::Context::Application(ctx) => {
            ctx.interaction
                .data
                .options
                .iter()
                .find(|option| option.name == "anime_title")
                .and_then(|option| option.value.as_str())
                .map(str::to_owned)
        }
        _ => None,
    };

    let Some(anime_title) = anime_title else {
        return Box::pin(stream::empty());
    };

    let anime_id = {
        let conn = ctx.data().db.lock().await;

        match database::get_anime_id_by_name(&conn, &anime_title) {
            Ok(Some(id)) => id,
            _ => return Box::pin(stream::empty()),
        }
    };

    let resources = {
        let conn = ctx.data().db.lock().await;

        match database::get_resource_titles_for_anime(&conn, anime_id) {
            Ok(resources) => resources,
            Err(_) => return Box::pin(stream::empty()),
        }
    };

    let titles: Vec<String> = resources
        .into_iter()
        .map(|(_, title)| title)
        .collect();

    let guesses = fuzzy_autocomplete(&titles, partial);

    Box::pin(stream::iter(guesses))
}


async fn autocomplete_anime<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let anime_names = {
        let guard = ctx.data().anime_names.lock().await;
        guard.clone()
    };

    let guesses = fuzzy_autocomplete(&anime_names, partial);
    stream::iter(guesses)
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn unlink_resource(
    ctx: Context<'_>,
    #[description = "The name of the anime"] 
    #[autocomplete = "autocomplete_anime"]
    anime_title: String,
    #[description = "The name of the resource"] 
    #[autocomplete = "autocomplete_resource"]
    resource_title: String,
) -> Result<(), Error> {
    if !is_remove_authorized(ctx).await? { return Ok(()) }
    let conn: tokio::sync::MutexGuard<'_, rusqlite::Connection> = ctx.data().db.lock().await;

    let anime_id = database::get_anime_id_by_name(&conn, &anime_title)?.expect("Error getting anime ID");
    let resource_id = database::get_resource_id_by_name(&conn, &resource_title)?.expect("Error getting resource id");

    let res = database::unlink_media_resource(&conn, resource_id, anime_id, MediaType::ANIME);
    match res {
        Ok(_) => {
            let message = format!("Successfully unlinked {} from {}.", resource_title, anime_title);
            ctx.send(CreateReply::default().content(message).ephemeral(true)).await?;
            return Ok(())
        },
        Err(e) => {
            let message = format!("Error unlinking {} from {}.", resource_title, anime_title);
            ctx.send(CreateReply::default().content(message).ephemeral(true)).await?;
            eprintln!("{}", e);
            return Ok(());
        },
    }
}