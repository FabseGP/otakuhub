use crate::apis::jikan::anime::model::*;
use leptos::prelude::*;

#[server]
pub async fn search_engine(query: String) -> Result<Vec<Anime>, ServerFnError> {
    use crate::apis::jikan::anime::fetch::*;
    Ok(search_anime(query).await.unwrap())
}
