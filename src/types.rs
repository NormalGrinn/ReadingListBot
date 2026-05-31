use poise::ChoiceParameter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ChoiceParameter)]
pub enum ResourceType {
    INTERVIEW,
    ANALYSIS
}


#[derive(Debug, Serialize, Deserialize, ChoiceParameter)]
pub enum Language {
    English,
    Japanese
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Season {
    WINTER,
    SPRING,
    SUMMER,
    FALL
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    TV,
    TV_SHORT,
    MOVIE,
    SPECIAL,
    OVA,
    ONA,
    MUSIC,
    MANGA,
    NOVEL,
    ONE_SHOT
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaSource {
    ORIGINAL,
    MANGA,
    LIGHT_NOVEL,
    VISUAL_NOVEL,
    VIDEO_GAME,
    OTHER,
    NOVEL,
    DOUJINSHI,
    ANIME,
    WEB_NOVEL,
    LIVE_ACTION,
    GAME,
    COMIC,
    MULTIMEDIA_PROJECT,
    PICTURE_BOOK,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Media {
    Anime(Anime),
}

#[derive(Debug, Serialize, Deserialize, ChoiceParameter)]
pub enum MediaType {
    ANIME,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub(crate) romaji: Option<String>,
    pub(crate) native: Option<String>,
    pub(crate) english: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub id: i32,
    pub title: Title,
    pub format: Option<Format>,
    pub season: Option<Season>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i32>,
    pub source: Option<MediaSource>,
    pub synonyms: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub link: String,
    pub title: String,
    pub synopsis: Option<String>,
    pub resource_type: ResourceType,
    pub langauge: Option<Language>,
    pub author: Option<String>,
    pub people: Vec<String>,
    pub related_media: Option<Vec<Media>>,
    pub tags: Vec<String>,
}