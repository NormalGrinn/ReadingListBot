use poise::CreateReply;
use rusqlite::Result;
use serenity::futures::{self, Stream};

use crate::{Context, Error, database::{get_resource_by_id, get_resource_id_by_name}, helpers::{create_resource_show_embed, fuzzy_autocomplete}};

use futures::stream::{self};

async fn autocomplete_resource<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let resource_titles = {
        let guard = ctx.data().resource_titles.lock().await;
        guard.clone()
    };

    let guesses = fuzzy_autocomplete(&resource_titles, partial);
    stream::iter(guesses)
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn show_resource(
    ctx: Context<'_>,
    #[description = "The name of the resource"] 
    #[autocomplete = "autocomplete_resource"]
    resource_title: String,
) -> Result<(), Error> {
    let conn: tokio::sync::MutexGuard<'_, rusqlite::Connection> = ctx.data().db.lock().await;
    let resource_id = match get_resource_id_by_name(&conn, &resource_title) {
        Ok(Some(id)) => id,
        Ok(None) => {
            ctx.send(CreateReply::default().content("Could not find resource in the DB by name.").ephemeral(true)).await?;
            return Ok(());
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error finding resource in the DB by name.").ephemeral(true)).await?;
            eprintln!("{}", e);
            return Ok(());
        },
    };

    match get_resource_by_id(&conn, resource_id) {
        Ok(resource) => {
            let embed = create_resource_show_embed(resource);
            ctx.send(CreateReply::default().embed(embed)).await?;
            return Ok(());
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error finding resource by ID.").ephemeral(true)).await?;
            eprintln!("{}", e);
            return Ok(());
        },
    }
}