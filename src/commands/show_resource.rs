use poise::CreateReply;
use rusqlite::Result;
use serenity::futures::{self, Stream};

use crate::{Context, Error, helpers::fuzzy_autocomplete};

async fn autocomplete_resource<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let guesses = fuzzy_autocomplete(&ctx.data().resource_titles, partial);
    futures::stream::iter(guesses)
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn add_anime(
    ctx: Context<'_>,
    #[description = "The name of the resource"] resource_title: String,
) -> Result<(), Error> {
    
    Ok(())
}