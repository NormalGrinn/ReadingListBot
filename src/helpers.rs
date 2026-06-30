use poise::CreateReply;
use rust_fuzzy_search::fuzzy_compare;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::colour::Colour;


use crate::types::{Format, MediaSource, Season, Title};
use crate::{Context, Error};

const MAX_DESCRIPTION_CHARS: usize = 3800; // margin under Discord's 4096 cap
const MAX_LINES_PER_PAGE: usize = 15;

pub async fn is_add_authorized(ctx: Context<'_>) -> Result<bool, Error> {
    if !ctx.data().add_users.contains(&ctx.author().id.get()) {
        ctx.send(CreateReply::default().content("You are not authorized to use this command.").ephemeral(true)).await?;
        return Ok(false);
    }
    Ok(true)
}

pub fn create_base_anime_embed(title: Title, id: i32, format: Option<Format>, season: Option<Season>, 
                               source: Option<MediaSource>, synonyms: Vec<String>, cover_image: Option<String>) -> CreateEmbed {
    let display_title = title
        .romaji
        .clone()
        .or_else(|| title.english.clone())
        .unwrap_or_else(|| "Unknown Title".to_string());
    let mut embed = CreateEmbed::new()
        .title(display_title)
        .colour(Colour::MAGENTA)
        .field("AL ID", id.to_string(), true);
    if let Some(cover_image) = cover_image { embed = embed.thumbnail(cover_image)}
    if let Some(format) = format { embed = embed.field("Format", format.to_string(), true); }
    if let Some(season) = season { embed = embed.field("Season", season.to_string(), true); }
    if let Some(source) = source { embed = embed.field("Source", source.to_string(), true); }
    if !synonyms.is_empty() {
        let synonyms_text = synonyms.join(", ");
        embed = embed.field("Synonyms", synonyms_text, false);
    }
    embed
}

pub fn paginate_lines(lines: &[String]) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut current_page = String::new();
    let mut current_line_count = 0;

    for line in lines {
        let would_exceed_chars = current_page.len() + line.len() + 1 > MAX_DESCRIPTION_CHARS;
        let would_exceed_lines = current_line_count >= MAX_LINES_PER_PAGE;

        if (would_exceed_chars || would_exceed_lines) && !current_page.is_empty() {
            pages.push(std::mem::take(&mut current_page));
            current_line_count = 0;
        }

        if !current_page.is_empty() {
            current_page.push('\n');
        }
        current_page.push_str(line);
        current_line_count += 1;
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

pub fn build_resource_embeds(base_embed: &CreateEmbed, pages: &[String]) -> Vec<CreateEmbed> {
    if pages.is_empty() {
        return vec![base_embed.clone()];
    }

    pages
        .iter()
        .enumerate()
        .map(|(i, page_text)| {
            base_embed
                .clone()
                .description(page_text)
                .footer(CreateEmbedFooter::new(format!(
                    "Page {}/{}",
                    i + 1,
                    pages.len()
                )))
        })
        .collect()
}

pub fn fuzzy_autocomplete<'a>(names: &[String], partial: &str) -> Vec<String> {
    let mut similarity_tuples: Vec<(String, f32)> = names
        .iter()
        .filter(|s| s.len() <= 100)
        .map(|s| (s.clone(), fuzzy_compare(&partial.to_lowercase(), &s.to_lowercase())))
        .collect();

    similarity_tuples.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    similarity_tuples
        .into_iter()
        .map(|(s, _)| s)
        .take(25)
        .collect()
}