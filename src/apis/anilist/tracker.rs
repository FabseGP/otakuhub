use leptos::prelude::*;

#[server]
pub async fn anilist_scraping(interval: u64) -> Result<(), ServerFnError> {
    use crate::{
        apis::anilist::{
            anime::fetch::all_anime, character::fetch::all_character, manga::fetch::all_manga,
        },
        config::types::MEILISEARCH_CLIENT,
    };
    use tokio::{
        task::spawn,
        time::{Duration, sleep},
    };
    let interval = Duration::from_hours(interval);
    let meilisearch_client = MEILISEARCH_CLIENT.get().unwrap();
    let anime_data = meilisearch_client.index("anilist_anime");
    let manga_data = meilisearch_client.index("anilist_manga");
    let character_data = meilisearch_client.index("anilist_character");
    spawn(async move {
        loop {
            sleep(interval).await;
            let anime_sources = all_anime().await.unwrap();
            anime_data
                .add_or_replace(&anime_sources, Some("id"))
                .await
                .unwrap();
            let character_sources = all_character().await.unwrap();
            character_data
                .add_or_replace(&character_sources, Some("id"))
                .await
                .unwrap();
            let manga_sources = all_manga().await.unwrap();
            manga_data
                .add_or_replace(&manga_sources, Some("id"))
                .await
                .unwrap();
        }
    });

    Ok(())
}
