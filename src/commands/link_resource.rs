use poise::CreateReply;
use rusqlite::Result;

use crate::{Context, Error, database::{get_media_id_by_al_id, insert_resource_media}, helpers::is_add_authorized, types::MediaType};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn link_resource(
    ctx: Context<'_>,
    #[description = "The type of the media you want to link"] media_type: MediaType,
    #[description = "The ID of the media you want to link (not using the media ID, but rather the AL ID)"] media_id: i32,
    #[description = "The ID of the resource you want to link"] resource_id: i32,
) -> Result<(), Error> {

    if !is_add_authorized(ctx).await? { return Ok(()) }

    let result = {
        let conn = ctx.data().db.lock().await;
        match media_type {
            MediaType::ANIME => {
                match get_media_id_by_al_id(&conn, media_id)? {
                    Some(internal_id) => insert_resource_media(&conn, resource_id, internal_id),
                    None => {
                        drop(conn);
                        ctx.send(CreateReply::default().content("Anime not found in database, add it first.").ephemeral(true)).await?;
                        return Ok(());
                    }
                }
            }
        }
    };

    match result {
        Ok(_) => {
            ctx.send(CreateReply::default().content("Resource linked successfully.").ephemeral(true)).await?;
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error linking resource.").ephemeral(true)).await?;
            eprintln!("Error linking resource: {}", e);
        }
    }

    Ok(())
}