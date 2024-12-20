use crate::apis::jikan::anime::model::Anime;
use leptos::prelude::*;

#[server]
pub async fn search_engine(query: String) -> Result<Vec<Anime>, ServerFnError> {
    use crate::apis::jikan::anime::fetch::*;
    /*  let results = if let Some(category) = category {
        match category.as_str() {
            "anime" => search_anime(query).await.unwrap(),
            "manga" => search_manga(query).await.unwrap(),
            "character" => search_character(query).await.unwrap(),
            _ => search_anime(query).await.unwrap(),
        }
    } else {
        search_anime(query).await.unwrap()
    };
    Ok(results) */

    Ok(search_anime(query).await.unwrap())
}
