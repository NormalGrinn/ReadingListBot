use poise::CreateReply;
use rusqlite::Result;

use crate::{Context, Error, al_queries::get_anime::get_anime, database::insert_anime, helpers::is_add_authorized};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn add_anime(
    ctx: Context<'_>,
    #[description = "The ID of the anime you want to add"] anime_id: i32,
) -> Result<(), Error> {

    if !is_add_authorized(ctx).await? { return Ok(()) }

    let res = get_anime(anime_id).await;
    match res {
        Ok(anime) => {
            let conn = ctx.data().db.lock().await;
                match insert_anime(&conn, &anime) {
                Ok(_) => {
                    let mut anime_names = ctx.data().anime_names.lock().await;
                    anime_names.push(anime.title.to_string());
                    anime_names.sort();
                    anime_names.dedup();
                    let response = format!("Added anime: {}", anime);
                    ctx.send(CreateReply::default().content(response).ephemeral(true)).await?;
                },
                Err(e) => {
                    ctx.send(CreateReply::default().content("Error adding anime to database").ephemeral(true)).await?;
                    eprintln!("Error inserting anime into DB: {}", e);
                }
            }
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error in querying the anime from AL").ephemeral(true)).await?;
            eprintln!("Error fetching anime from AL: {}", e);
        }
    }
    Ok(())
}