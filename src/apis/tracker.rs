use leptos::prelude::*;

#[server]
pub async fn start_api_scraping_tasks(fetch_interval: u64) -> Result<(), ServerFnError> {
    use crate::apis::{anilist::tracker::*, jikan::tracker::*};
    jikan_scraping(fetch_interval).await.unwrap();
    anilist_scraping(fetch_interval).await.unwrap();

    Ok(())
}
