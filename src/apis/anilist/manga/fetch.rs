use crate::apis::anilist::manga::model::Manga;
use leptos::prelude::*;

#[server]
pub async fn search_manga(query: String) -> Result<Vec<Manga>, ServerFnError> {
    use crate::config::types::MEILISEARCH_CLIENT;
    if let Some(meilisearch_client) = MEILISEARCH_CLIENT.get()
        && let Ok(index) = meilisearch_client.get_index("jikan_manga").await
        && let Ok(results) = index
            .search()
            .with_query(query.as_str())
            .execute::<Manga>()
            .await
    {
        Ok(results.hits.into_iter().map(|s| s.result).collect())
    } else {
        search_manga_api(query).await
    }
}

#[server]
pub async fn search_manga_api(query: String) -> Result<Vec<Manga>, ServerFnError> {
    use crate::{apis::anilist::manga::model::MangaResponse, config::types::HTTP_CLIENT};
    let request_url = format!("https://api.jikan.moe/v4/manga?q={query}");
    match HTTP_CLIENT.get(request_url).send().await {
        Ok(resp) => match resp.json::<MangaResponse>().await {
            Ok(data) if !data.data.is_empty() => Ok(data.data),
            Ok(_) | Err(_) => Ok(Vec::default()),
        },
        Err(_) => Ok(Vec::default()),
    }
}

#[server]
pub async fn all_manga() -> Result<Vec<Manga>, ServerFnError> {
    use crate::{apis::anilist::manga::model::MangaResponse, config::types::HTTP_CLIENT};
    use leptos::logging::log;
    match HTTP_CLIENT
        .get("https://api.jikan.moe/v4/manga")
        .send()
        .await
    {
        Ok(resp) => match resp.json::<MangaResponse>().await {
            Ok(data) if !data.data.is_empty() => {
                let mut all_manga: Vec<Manga> = Vec::with_capacity(data.pagination.items.total);
                let mut current_page = 1;
                loop {
                    let request_url = format!("https://api.jikan.moe/v4/manga?page={current_page}");
                    match HTTP_CLIENT.get(&request_url).send().await {
                        Ok(resp) => match resp.json::<MangaResponse>().await {
                            Ok(data) => {
                                all_manga.extend(data.data);
                                if !data.pagination.has_next_page {
                                    break;
                                }
                                current_page += 1;
                            }
                            Err(e) => {
                                log!("Parsing error: {e}");
                                break;
                            }
                        },
                        Err(e) => {
                            log!("Request error: {e}");
                            break;
                        }
                    }
                }
                Ok(all_manga)
            }
            Ok(_) | Err(_) => Ok(Vec::default()),
        },
        Err(_) => Ok(Vec::default()),
    }
}
