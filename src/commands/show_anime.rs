use poise::CreateReply;
use rusqlite::Result;

use crate::{Context, Error, database::{get_anime_by_al_id, get_resources_for_anime}};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn show_anime(
    ctx: Context<'_>,
    #[description = "The AniList ID of the anime"] anime_id: i32,
) -> Result<(), Error> {
    let (anime, resources) = {
        let conn = ctx.data().db.lock().await;
        let anime = get_anime_by_al_id(&conn, anime_id)?;
        let resources = get_resources_for_anime(&conn, anime_id)?;
        (anime, resources)
    };

    match anime {
        None => {
            ctx.send(CreateReply::default().content("Anime not found in database.").ephemeral(true)).await?;
        }
        Some(anime) => {
            let mut response = format!("{}\n\n", anime);

            if resources.is_empty() {
                response.push_str("No resources linked yet.");
            } else {
                response.push_str("**Resources:**\n");
                for r in resources {
                    let author = r.author.map(|a| format!(" by {}", a)).unwrap_or_default();
                    let language = r.langauge.map(|l| format!(" [{}]", l)).unwrap_or_default();
                    response.push_str(&format!("- [{}]({}){}{}\n", r.title, r.link, author, language));
                }
            }

            ctx.send(CreateReply::default().content(response).ephemeral(false)).await?;
        }
    }

    Ok(())
}