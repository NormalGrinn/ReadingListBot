
pub const GET_ANIME_QUERY: &str = r#"
query Query($mediaId: Int) {
  Media(id: $mediaId) {
    id
    title {
      romaji
      native
      english
    }
    format
    season
    seasonYear
    source
    synonyms
    id
    coverImage {
      medium
    }
  }
}
"#;
