use std::time::Duration;

use poise::CreateReply;
use rusqlite::Result;
use rust_fuzzy_search::fuzzy_compare;
use serenity::futures;
use ::serenity::futures::Stream;

use crate::{Context, Error, database::{self, get_anime_by_al_id, get_resources_for_anime}, helpers};

struct EmbedResource {
    title: String,
    link: String,
    author: Option<String>,
}

impl std::fmt::Display for EmbedResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("[{}]({})", self.title, self.link);
        match &self.author {
            Some(auth) => {
                s = format!("{} by {}", s, auth);
                write!(f, "{}", s)
            },
            None => write!(f, "{}", s),
        }
    }
}

async fn autocomplete_anime<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let names = &ctx.data().anime_names;
    let mut similarity_tuples: Vec<(String, f32)> = names
        .iter()
        .filter(|s| s.len() <= 100)
        .map(|s| (s.clone(), fuzzy_compare(&partial.to_lowercase(), &s.to_lowercase())))
        .collect();
    similarity_tuples.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let guesses: Vec<String> = similarity_tuples
        .into_iter()
        .map(|(s, _)| s)
        .take(25) 
        .collect();
    futures::stream::iter(guesses)
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn show_anime(
    ctx: Context<'_>,
    #[description = "The AniList ID of the anime"] 
    #[autocomplete = "autocomplete_anime"]
    anime_name: String,
) -> Result<(), Error> {
    let conn = ctx.data().db.lock().await;
    let anime_id: i32;
    match database::get_anime_id_by_name(&conn, &anime_name) {
        Ok(opt_id) => match opt_id {
            Some(id) => anime_id = id,
            None => {
                ctx.send(CreateReply::default().content("This anime has no ID.").ephemeral(true)).await?;
                return Ok(())
            },
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error finding the anime's ID.").ephemeral(true)).await?;
            eprintln!("Error finding the anime's ID, {}", e);
            return Ok(())
        },
    }
    let (anime, resources) = {
        let anime = get_anime_by_al_id(&conn, anime_id)?;
        let resources = get_resources_for_anime(&conn, anime_id)?;
        (anime, resources)
    };
    drop(conn);

    let anime = match anime {
        None => {
            ctx.send(CreateReply::default().content("Anime not found in database.").ephemeral(true)).await?;
            return Ok(())
        }
        Some(anime) => anime,
    };

    let base_embed = helpers::create_base_anime_embed(
        anime.title, anime.id, anime.format, anime.season, anime.source,
        anime.synonyms, anime.cover_image_small,
    );

    let embed_resources: Vec<EmbedResource> = resources
        .into_iter()
        .map(|r| EmbedResource { title: r.title, link: r.link, author: r.author })
        .collect();

    let lines: Vec<String> = embed_resources.iter().map(|r| r.to_string()).collect();
    let pages = helpers::paginate_lines(&lines);
    let embeds = helpers::build_resource_embeds(&base_embed, &pages);

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

    // disable buttons after timeout
    msg.edit(
        ctx,
        CreateReply::default()
            .embed(embeds[current_page].clone())
            .components(vec![nav_row_disabled()]),
    )
    .await?;


    Ok(())
}

fn nav_row(page: usize, total: usize) -> serenity::builder::CreateActionRow {
    serenity::builder::CreateActionRow::Buttons(vec![
        serenity::builder::CreateButton::new("prev_page")
            .label("◀ Prev")
            .style(serenity::model::application::ButtonStyle::Secondary)
            .disabled(page == 0),
        serenity::builder::CreateButton::new("next_page")
            .label("Next ▶")
            .style(serenity::model::application::ButtonStyle::Secondary)
            .disabled(page + 1 >= total),
    ])
}

fn nav_row_disabled() -> serenity::builder::CreateActionRow {
    serenity::builder::CreateActionRow::Buttons(vec![
        serenity::builder::CreateButton::new("prev_page").label("◀ Prev").style(serenity::model::application::ButtonStyle::Secondary).disabled(true),
        serenity::builder::CreateButton::new("next_page").label("Next ▶").style(serenity::model::application::ButtonStyle::Secondary).disabled(true),
    ])
}