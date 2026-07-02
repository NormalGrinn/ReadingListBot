use poise::CreateReply;
use rust_fuzzy_search::fuzzy_compare;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::colour::Colour;


use crate::types::{Format, MediaSource, Resource, Season, Title};
use crate::{Context, Error};

const MAX_DESCRIPTION_CHARS: usize = 3800; // margin under Discord's 4096 cap
const MAX_LINES_PER_PAGE: usize = 30;
const MAX_FIELD_VALUE: usize = 1000;

fn truncate_to_chars(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        s.chars().take(max_chars).collect()
    }
}

pub async fn is_add_authorized(ctx: Context<'_>) -> Result<bool, Error> {
    if !ctx.data().add_users.contains(&ctx.author().id.get()) {
        ctx.send(CreateReply::default().content("You are not authorized to use this command.").ephemeral(true)).await?;
        return Ok(false);
    }
    Ok(true)
}

pub fn create_resource_show_embed(r: Resource) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(r.title)
        .color(Colour::MAGENTA)
        .field("Link", r.link, false)
        .field("Resource ID", r.resource_id.to_string(), true)
        .field("Resource type", r.resource_type.to_string(), true);
    if let Some(synopsis) = r.synopsis { embed = embed.description(truncate_to_chars(&synopsis, MAX_DESCRIPTION_CHARS)); };
    if let Some(langauge) = r.language { embed = embed.field("Language", langauge.to_string(), true) };
    if let Some(related_media) = r.related_media {
        if !related_media.is_empty() {
            let mut media: String = related_media
                .iter()
                .map(|m| format!("{} ({})", m.title, m.media_type))
                .collect::<Vec<_>>()
                .join("\n");
            media = truncate_to_chars(&media, MAX_FIELD_VALUE);
            embed = embed.field("Related media", media, true);
        }
    }

    if !r.people.is_empty() {
        let people = r.people.join(", ");
        embed = embed.field("People", truncate_to_chars(&people, MAX_FIELD_VALUE), true);
    }
    if !r.tags.is_empty() {
        let tags = r.tags.join(", ");
        embed = embed.field("Tags", truncate_to_chars(&tags, MAX_FIELD_VALUE), true);
    }
    embed
}

pub fn create_base_anime_embed(title: Title, media_id: i32, al_id: i32, format: Option<Format>, season: Option<Season>, year: Option<i32>,
                               source: Option<MediaSource>, synonyms: Vec<String>, cover_image: Option<String>) -> CreateEmbed {
    let display_title = title.to_string();

    let mut embed = CreateEmbed::new()
        .title(display_title)
        .colour(Colour::MAGENTA)
        .field("Media ID", media_id.to_string(), true)
        .field("AL ID", al_id.to_string(), true);
    if let Some(cover_image) = cover_image { embed = embed.thumbnail(cover_image)}
    if let Some(format) = format { embed = embed.field("Format", format.to_string(), true); }
    if !season.is_none() && !year.is_none() {
        let season_year = format!("{} {}", season.unwrap(), year.unwrap());
        embed = embed.field("Season", season_year, true);
    }
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

pub fn nav_row(page: usize, total: usize) -> serenity::builder::CreateActionRow {
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

pub fn nav_row_disabled() -> serenity::builder::CreateActionRow {
    serenity::builder::CreateActionRow::Buttons(vec![
        serenity::builder::CreateButton::new("prev_page").label("◀ Prev").style(serenity::model::application::ButtonStyle::Secondary).disabled(true),
        serenity::builder::CreateButton::new("next_page").label("Next ▶").style(serenity::model::application::ButtonStyle::Secondary).disabled(true),
    ])
}