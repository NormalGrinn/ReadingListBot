use crate::types::*;
use std::fmt;

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceType::INTERVIEW => "INTERVIEW",
            ResourceType::ANALYSIS => "ANALYSIS",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::English => "ENGLISH",
            Language::Japanese => "JAPANESE",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Season::WINTER => "WINTER",
            Season::SPRING => "SPRING",
            Season::SUMMER => "SUMMER",
            Season::FALL => "FALL",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Format::TV => "TV",
            Format::TV_SHORT => "TV_SHORT",
            Format::MOVIE => "MOVIE",
            Format::SPECIAL => "SPECIAL",
            Format::OVA => "OVA",
            Format::ONA => "ONA",
            Format::MUSIC => "MUSIC",
            Format::MANGA => "MANGA",
            Format::NOVEL => "NOVEL",
            Format::ONE_SHOT => "ONE_SHOT",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for MediaSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaSource::ORIGINAL => "ORIGINAL",
            MediaSource::MANGA => "MANGA",
            MediaSource::LIGHT_NOVEL => "LIGHT_NOVEL",
            MediaSource::VISUAL_NOVEL => "VISUAL_NOVEL",
            MediaSource::VIDEO_GAME => "VIDEO_GAME",
            MediaSource::OTHER => "OTHER",
            MediaSource::NOVEL => "NOVEL",
            MediaSource::DOUJINSHI => "DOUJINSHI",
            MediaSource::ANIME => "ANIME",
            MediaSource::WEB_NOVEL => "WEB_NOVEL",
            MediaSource::LIVE_ACTION => "LIVE_ACTION",
            MediaSource::GAME => "GAME",
            MediaSource::COMIC => "COMIC",
            MediaSource::MULTIMEDIA_PROJECT => "MULTIMEDIA_PROJECT",
            MediaSource::PICTURE_BOOK => "PICTURE_BOOK",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = format!("{}", self.id);

        let title = self
            .title
            .english
            .as_deref()
            .or(self.title.romaji.as_deref())
            .or(self.title.native.as_deref())
            .unwrap_or("Unknown Title");

        let format = match &self.format {
            Some(fm) => format!("{fm:?}"),
            None => "Unknown".to_string(),
        };

        let season = match &self.season {
            Some(s) => format!("{s:?}"),
            None => "Unknown".to_string(),
        };

        let year = self
            .season_year
            .map(|y| y.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let source = match &self.source {
            Some(src) => format!("{src:?}"),
            None => "Unknown".to_string(),
        };

        let synonyms = if self.synonyms.is_empty() {
            "None".to_string()
        } else {
            self.synonyms.join(", ")
        };

        write!(
            f,
            "ID: {}\nAnime: {}\nFormat: {}\nSeason: {} {}\nSource: {}\nSynonyms: {}",
            id, title, format, season, year, source, synonyms
        )
    }
}

impl std::str::FromStr for Season {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WINTER" => Ok(Season::WINTER),
            "SPRING" => Ok(Season::SPRING),
            "SUMMER" => Ok(Season::SUMMER),
            "FALL" => Ok(Season::FALL),
            _ => Err(format!("Unknown season: {}", s)),
        }
    }
}

impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TV" => Ok(Format::TV),
            "TV_SHORT" => Ok(Format::TV_SHORT),
            "MOVIE" => Ok(Format::MOVIE),
            "SPECIAL" => Ok(Format::SPECIAL),
            "OVA" => Ok(Format::OVA),
            "ONA" => Ok(Format::ONA),
            "MUSIC" => Ok(Format::MUSIC),
            "MANGA" => Ok(Format::MANGA),
            "NOVEL" => Ok(Format::NOVEL),
            "ONE_SHOT" => Ok(Format::ONE_SHOT),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

impl std::str::FromStr for MediaSource {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ORIGINAL" => Ok(MediaSource::ORIGINAL),
            "MANGA" => Ok(MediaSource::MANGA),
            "LIGHT_NOVEL" => Ok(MediaSource::LIGHT_NOVEL),
            "VISUAL_NOVEL" => Ok(MediaSource::VISUAL_NOVEL),
            "VIDEO_GAME" => Ok(MediaSource::VIDEO_GAME),
            "OTHER" => Ok(MediaSource::OTHER),
            "NOVEL" => Ok(MediaSource::NOVEL),
            "DOUJINSHI" => Ok(MediaSource::DOUJINSHI),
            "ANIME" => Ok(MediaSource::ANIME),
            "WEB_NOVEL" => Ok(MediaSource::WEB_NOVEL),
            "LIVE_ACTION" => Ok(MediaSource::LIVE_ACTION),
            "GAME" => Ok(MediaSource::GAME),
            "COMIC" => Ok(MediaSource::COMIC),
            "MULTIMEDIA_PROJECT" => Ok(MediaSource::MULTIMEDIA_PROJECT),
            "PICTURE_BOOK" => Ok(MediaSource::PICTURE_BOOK),
            _ => Err(format!("Unknown media source: {}", s)),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "English" => Ok(Language::English),
            "Japanese" => Ok(Language::Japanese),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

impl std::str::FromStr for ResourceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INTERVIEW" => Ok(ResourceType::INTERVIEW),
            "ANALYSIS" => Ok(ResourceType::ANALYSIS),
            _ => Err(format!("Unknown resource type: {}", s)),
        }
    }
}