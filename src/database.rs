use rusqlite::Connection;
use crate::types::{Anime, Language, Resource, ResourceType, Title};

pub fn insert_anime(conn: &Connection, anime: &Anime) -> Result<i64, rusqlite::Error> {
    let synonyms = anime.synonyms.join(",");
    let title_json = serde_json::to_string(&anime.title).expect("Failed to serialize title");

    let tx = conn.unchecked_transaction()?;

    tx.execute("INSERT INTO media (media_type) VALUES ('ANIME')", [])?;
    let media_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO anime (media_id, al_id, title, format, season, seasonYear, source, synonyms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            media_id,
            anime.id,
            title_json,
            anime.format.as_ref().map(|f| f.to_string()),
            anime.season.as_ref().map(|s| s.to_string()),
            anime.season_year,
            anime.source.as_ref().map(|s| s.to_string()),
            synonyms
        ],
    )?;

    tx.commit()?;
    Ok(media_id)
}

pub fn insert_resource(conn: &Connection, url: &str, resource_title: &str, resource_type: &ResourceType,
     resource_synopsis: Option<&str>, resource_language: Option<&Language>, resource_author: Option<&str>) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO resources (link, resource_title, resource_synopsis, resource_type, resource_language, author)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            url,
            resource_title,
            resource_synopsis,
            resource_type.to_string(),
            resource_language.map(|l| l.to_string()),
            resource_author,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_media_id_by_al_id(conn: &Connection, al_id: i32) -> Result<Option<i64>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT media_id FROM anime WHERE al_id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![al_id])?;
    match rows.next()? {
        Some(row) => Ok(Some(row.get(0)?)),
        None => Ok(None),
    }
}

pub fn insert_resource_media(conn: &Connection, resource_id: i32, media_id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO resource_media (resource_id, media_id) VALUES (?1, ?2)",
        rusqlite::params![resource_id, media_id],
    )?;
    Ok(())
}

pub fn get_anime_by_al_id(conn: &Connection, al_id: i32) -> Result<Option<Anime>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT al_id, title, format, season, seasonYear, source, synonyms FROM anime WHERE al_id = ?1"
    )?;
    let mut rows = stmt.query(rusqlite::params![al_id])?;
    match rows.next()? {
        Some(row) => {
            let title_json: String = row.get(1)?;
            let title: Title = serde_json::from_str(&title_json)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?;

            Ok(Some(Anime {
                id: row.get(0)?,
                title,
                format: row.get::<_, Option<String>>(2)?.and_then(|s| s.parse().ok()),
                season: row.get::<_, Option<String>>(3)?.and_then(|s| s.parse().ok()),
                season_year: row.get(4)?,
                source: row.get::<_, Option<String>>(5)?.and_then(|s| s.parse().ok()),
                synonyms: row.get::<_, Option<String>>(6)?
                    .map(|s| s.split(',').map(String::from).collect())
                    .unwrap_or_default(),
            }))
        },
        None => Ok(None),
    }
}

fn get_people_for_resource(conn: &Connection, resource_id: i64) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("
        SELECT p.person_name FROM people p
        JOIN resource_people rp ON p.person_id = rp.person_id
        WHERE rp.resource_id = ?1
    ")?;
    let rows = stmt.query_map(rusqlite::params![resource_id], |row| row.get(0))?;
    rows.collect()
}

fn get_tags_for_resource(conn: &Connection, resource_id: i64) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("
        SELECT t.tag_name FROM tags t
        JOIN resource_tags rt ON t.tag_id = rt.tag_id
        WHERE rt.resource_id = ?1
    ")?;
    let rows = stmt.query_map(rusqlite::params![resource_id], |row| row.get(0))?;
    rows.collect()
}

pub fn get_resources_for_anime(conn: &Connection, al_id: i32) -> Result<Vec<Resource>, rusqlite::Error> {
    let mut stmt = conn.prepare("
        SELECT r.resource_id, r.link, r.resource_title, r.resource_synopsis, r.resource_type, r.resource_language, r.author
        FROM resources r
        JOIN resource_media rm ON r.resource_id = rm.resource_id
        JOIN anime a ON rm.media_id = a.media_id
        WHERE a.al_id = ?1
    ")?;

    let resource_rows = stmt.query_map(rusqlite::params![al_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,  // resource_id
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, Option<String>>(6)?,
        ))
    })?.collect::<Result<Vec<_>, _>>()?;

    let mut resources = Vec::new();

    for (resource_id, link, title, synopsis, resource_type, language, author) in resource_rows {
        let people = get_people_for_resource(conn, resource_id)?;
        let tags = get_tags_for_resource(conn, resource_id)?;
        let related_media = None;

        resources.push(Resource {
            link,
            title,
            synopsis,
            resource_type: resource_type.parse().expect("Invalid resource type"),
            langauge: language.and_then(|l| l.parse().ok()),
            author,
            people,
            related_media,
            tags: tags,
        });
    }

    Ok(resources)
}