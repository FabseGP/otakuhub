use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterResponse {
    pub pagination: CharacterPagination,
    pub data: Vec<Character>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterPagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub items: CharacterItems,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterItems {
    pub count: i32,
    pub total: usize,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Character {
    pub mal_id: i32,
    pub url: String,
    pub images: CharacterImageTypes,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub about: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterImageTypes {
    pub webp: CharacterImageWebp,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterImageWebp {
    pub image_url: Option<String>,
}
