use poise::CreateReply;
use rusqlite::Result;

use crate::{Context, Error, database::{insert_resource}, helpers::is_add_authorized, types::{Language, ResourceType}};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn add_resource(
    ctx: Context<'_>,
    #[description = "The the url of the resource"] url: String,
    #[description = "Title of the resource"] resource_title: String,
    #[description = "The resource type"] resource_type: ResourceType,
    #[description = "An optional synopsis"] resource_synopsis: Option<String>,
    #[description = "Language of the resource"] resource_language: Option<Language>,
    #[description = "Author of the resource"] resource_author: Option<String>,
) -> Result<(), Error> {

    if !is_add_authorized(ctx).await? { return Ok(()) }

    if resource_title.len() > 100 {
        ctx.send(CreateReply::default().content("Title over 100 characters").ephemeral(true)).await?;
        return Ok(());
    }

    let result = {
        let conn: tokio::sync::MutexGuard<'_, rusqlite::Connection> = ctx.data().db.lock().await;
        insert_resource(
            &conn,
            &url,
            &resource_title,
            &resource_type,
            resource_synopsis.as_deref(),
            resource_language.as_ref(),
            resource_author.as_deref(),
        )
    };

    match result {
        Ok(_) => {
            let mut resource_titles = ctx.data().resource_titles.lock().await;
            resource_titles.push(resource_title.clone());
            resource_titles.sort();
            ctx.send(CreateReply::default().content("Successfully added resource to the database").ephemeral(true)).await?;
        },
        Err(e) => {
            ctx.send(CreateReply::default().content("Error adding resource to database").ephemeral(true)).await?;
            eprintln!("Error inserting anime into DB: {}", e);
        },
    }

    Ok(())
}
