use crate::apis::jikan::anime::model::Anime;
use leptos::prelude::*;

#[server]
pub async fn search_anime(query: String) -> Result<Vec<Anime>, ServerFnError> {
    use crate::config::types::MEILISEARCH_CLIENT;
    if let Some(meilisearch_client) = MEILISEARCH_CLIENT.get()
        && let Ok(index) = meilisearch_client.get_index("jikan_anime").await
        && let Ok(results) = index
            .search()
            .with_query(query.as_str())
            .with_attributes_to_search_on(&["titles.title"])
            .execute::<Anime>()
            .await
    {
        Ok(results.hits.into_iter().map(|s| s.result).collect())
    } else {
        search_anime_api(query).await
    }
}

#[server]
pub async fn search_anime_api(query: String) -> Result<Vec<Anime>, ServerFnError> {
    use crate::{
        apis::jikan::{
            anime::model::AnimeResponse,
            shared::{RATE_LIMITER_PER_MINUTE, RATE_LIMITER_PER_SECOND},
        },
        config::types::HTTP_CLIENT,
    };
    let request_url = format!("https://api.jikan.moe/v4/anime?q={query}");
    RATE_LIMITER_PER_SECOND.until_ready().await;
    RATE_LIMITER_PER_MINUTE.until_ready().await;
    match HTTP_CLIENT.get(request_url).send().await {
        Ok(resp) => match resp.json::<AnimeResponse>().await {
            Ok(data) if !data.data.is_empty() => Ok(data.data),
            Ok(_) | Err(_) => Ok(Vec::default()),
        },
        Err(_) => Ok(Vec::default()),
    }
}

#[server]
pub async fn all_anime() -> Result<Vec<Anime>, ServerFnError> {
    use crate::{
        apis::jikan::{
            anime::model::AnimeResponse,
            shared::{RATE_LIMITER_PER_MINUTE, RATE_LIMITER_PER_SECOND},
        },
        config::types::HTTP_CLIENT,
    };
    use futures::future::join_all;
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    let initial_resp = HTTP_CLIENT
        .get("https://api.jikan.moe/v4/anime")
        .send()
        .await?;

    let initial_data: AnimeResponse = initial_resp.json().await?;
    let total_pages = initial_data.pagination.last_visible_page;

    let semaphore = Arc::new(Semaphore::new(3));

    let fetch_page = move |page: i32| {
        let semaphore_clone = semaphore.clone();
        async move {
            let _permit = semaphore_clone.acquire().await.unwrap();

            RATE_LIMITER_PER_SECOND.until_ready().await;
            RATE_LIMITER_PER_MINUTE.until_ready().await;

            let request_url = format!("https://api.jikan.moe/v4/anime?page={page}");

            match HTTP_CLIENT.get(&request_url).send().await {
                Ok(resp) => resp
                    .json::<AnimeResponse>()
                    .await
                    .map(|data| data.data)
                    .ok(),
                Err(_) => None,
            }
        }
    };

    let page_futures: Vec<_> = (1..=total_pages).map(fetch_page).collect();
    let results = join_all(page_futures).await;
    let all_anime: Vec<Anime> = results.into_iter().flatten().flatten().collect();

    Ok(all_anime)
}
