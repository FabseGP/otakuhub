use crate::apis::anilist::anime::model::Anime;
use leptos::prelude::*;

#[server]
pub async fn search_anime(query: String) -> Result<Vec<Anime>, ServerFnError> {
    use crate::config::types::MEILISEARCH_CLIENT;
    if let Some(meilisearch_client) = MEILISEARCH_CLIENT.get()
        && let Ok(index) = meilisearch_client.get_index("jikan_anime").await
        && let Ok(results) = index
            .search()
            .with_query(query.as_str())
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
    use crate::config::types::HTTP_CLIENT;
    use graphql_client::{GraphQLQuery, Response};
    use leptos::logging::log;
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/apis/anilist/schema.json",
        query_path = "src/apis/anilist/anime/query.graphql",
        response_derives = "Debug"
    )]
    struct MediaQuery;
    let variables = media_query::Variables { search: query };
    match HTTP_CLIENT
        .post("https://graphql.anilist.co/")
        .json(&MediaQuery::build_query(variables))
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Response<media_query::ResponseData>>().await {
            Ok(parsed) => {
                if let Some(data) = parsed.data
                    && let Some(page) = data.page
                    && let Some(media) = page.media
                {
                    for anime in media.iter().flatten() {
                        log!("---");
                        log!("ID: {}", anime.id);

                        if let Some(title) = &anime.title {
                            log!("Romaji: {}", title.romaji.as_deref().unwrap_or("N/A"));
                            log!("English: {}", title.english.as_deref().unwrap_or("N/A"));
                            log!("Native: {}", title.native.as_deref().unwrap_or("N/A"));
                        }
                    }
                } else {
                    log!("No media found");
                }
                Ok(Vec::default())
            }
            Err(_) => {
                log!("No data received");
                Ok(Vec::default())
            }
        },
        Err(_) => Ok(Vec::default()),
    }
}

#[server]
pub async fn all_anime() -> Result<Vec<Anime>, ServerFnError> {
    use crate::{apis::anilist::anime::model::AnimeResponse, config::types::HTTP_CLIENT};
    use leptos::logging::log;
    match HTTP_CLIENT
        .get("https://api.jikan.moe/v4/anime")
        .send()
        .await
    {
        Ok(resp) => match resp.json::<AnimeResponse>().await {
            Ok(data) if !data.data.is_empty() => {
                let mut all_anime: Vec<Anime> = Vec::with_capacity(data.pagination.items.total);
                let mut current_page = 1;
                loop {
                    let request_url = format!("https://api.jikan.moe/v4/anime?page={current_page}");
                    match HTTP_CLIENT.get(&request_url).send().await {
                        Ok(resp) => match resp.json::<AnimeResponse>().await {
                            Ok(data) => {
                                all_anime.extend(data.data);
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
                Ok(all_anime)
            }
            Ok(_) | Err(_) => Ok(Vec::default()),
        },
        Err(_) => Ok(Vec::default()),
    }
}
