use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaResponse {
    pub data: Vec<Manga>,
    pub pagination: MangaPagination,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaPagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub items: MangaItems,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaItems {
    pub count: i32,
    pub total: usize,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Manga {
    pub mal_id: i32,
    pub url: String,
    pub images: MangaImageTypes,
    pub approved: bool,
    pub trailer: MangaTrailerInfo,
    pub titles: Vec<MangaTitleTypes>,
    #[serde(rename = "type")]
    pub manga_type: Option<String>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub status: String,
    pub publishing: bool,
    pub published: MangaAired,
    pub score: Option<f32>,
    pub rank: Option<i32>,
    pub popularity: Option<i32>,
    pub favorites: Option<i32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub genres: Vec<MangaGenres>,
    pub authors: Vec<MangaAuthors>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaImageTypes {
    pub webp: MangaImageWebp,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaImageWebp {
    pub image_url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaTrailerInfo {
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaTitleTypes {
    #[serde(rename = "type")]
    pub title_type: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaAired {
    #[serde(rename = "string")]
    pub aired_string: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaGenres {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MangaAuthors {
    pub name: String,
}
