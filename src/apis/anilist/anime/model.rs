use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeResponse {
    pub pagination: AnimePagination,
    pub data: Vec<Anime>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimePagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub items: AnimeItems,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeItems {
    pub count: i32,
    pub total: usize,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Anime {
    pub mal_id: i32,
    pub url: String,
    pub images: AnimeImageTypes,
    pub trailer: AnimeTrailerInfo,
    pub titles: Vec<AnimeTitleTypes>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub source: Option<String>,
    pub episodes: Option<i32>,
    pub status: String,
    pub aired: AnimeAired,
    pub duration: Option<String>,
    pub score: Option<f32>,
    pub rank: Option<i32>,
    pub popularity: Option<i32>,
    pub favorites: Option<i32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub genres: Vec<AnimeGenres>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeImageTypes {
    pub webp: AnimeImageWebp,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeImageWebp {
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeTrailerInfo {
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeTitleTypes {
    #[serde(rename = "type")]
    pub title_type: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeAired {
    #[serde(rename = "string")]
    pub aired_string: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AnimeGenres {
    pub name: String,
}
