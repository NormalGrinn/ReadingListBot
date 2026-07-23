use std::time::Duration;

use poise::CreateReply;
use rusqlite::Result;
use serenity::model::Colour;

use crate::{Context, Error, database::{self}, helpers::{build_resource_embeds, nav_row, nav_row_disabled, paginate_lines}};

// const PAGE_SIZE: usize = 10;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn list_resources(
    ctx: Context<'_>,
    #[description = "Should resources that are already linked be filtered?"] filter: Option<bool>,
) -> Result<(), Error> {
    let conn = ctx.data().db.lock().await;
    let resources: Vec<(i32, String)>;
    let f = filter.unwrap_or(false);
    if f {
        resources = database::get_all_resource_ids_and_titles_filtered(&conn)?;
    } else {
        resources = database::get_all_resource_ids_and_titles(&conn)?;
    }
    drop(conn);

    if resources.is_empty() {
        ctx.send(CreateReply::default().content("No resources found.").ephemeral(true)).await?;
        return Ok(());
    }

    let lines: Vec<String> = resources
        .iter()
        .map(|(id, title)| format!("`{id}` — {title}"))
        .collect();


    let pages = paginate_lines(&lines);
    let base_embed = serenity::builder::CreateEmbed::default().title("Resources").colour(Colour::MAGENTA);
    let embeds = build_resource_embeds(&base_embed, &pages);

    let mut current_page = 0;
    let reply = if embeds.len() > 1 {
        CreateReply::default()
            .embed(embeds[current_page].clone())
            .components(vec![nav_row(current_page, embeds.len())])
    } else {
        CreateReply::default().embed(embeds[current_page].clone())
    };

    let msg = ctx.send(reply).await?;

    if embeds.len() <= 1 {
        return Ok(());
    }

    let reply_msg = msg.message().await?;

    while let Some(interaction) = reply_msg
        .await_component_interaction(ctx.serenity_context())
        .timeout(Duration::from_secs(120))
        .author_id(ctx.author().id)
        .await
    {
        match interaction.data.custom_id.as_str() {
            "prev_page" if current_page > 0 => current_page -= 1,
            "next_page" if current_page + 1 < embeds.len() => current_page += 1,
            _ => {}
        }

        interaction
            .create_response(
                ctx.serenity_context(),
                serenity::builder::CreateInteractionResponse::UpdateMessage(
                    serenity::builder::CreateInteractionResponseMessage::new()
                        .embed(embeds[current_page].clone())
                        .components(vec![nav_row(current_page, embeds.len())]),
                ),
            )
            .await?;
    }

    msg.edit(
        ctx,
        CreateReply::default()
            .embed(embeds[current_page].clone())
            .components(vec![nav_row_disabled()]),
    )
    .await?;

    Ok(())
}