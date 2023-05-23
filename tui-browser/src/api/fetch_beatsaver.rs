use crate::types::map_types::{Map, Maps};

pub async fn fetch_maps(
    query: &str,
    page_index: i32,
) -> Result<Vec<Map>, Box<dyn std::error::Error>> {
    let resp: Maps = reqwest::get(format!(
        "https://api.beatsaver.com/search/text/{}?q={}&sortOrder=Relevance",
        page_index, query
    ))
    .await?
    .json()
    .await?;
    Ok(resp.docs)
}

pub async fn fetch_map_details(id: &String) -> Result<Map, Box<dyn std::error::Error>> {
    let resp: Map = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .json()
        .await?;
    Ok(resp)
}
