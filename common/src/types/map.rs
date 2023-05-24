use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub id: String,
    pub name: String,
    pub last_published_at: String,
    pub metadata: Metadata,
    pub stats: Stats,
    pub description: String,
    pub ranked: bool,
    pub qualified: bool,
    pub versions: Vec<Version>,
    pub automapper: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub bpm: f32,
    pub duration: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub downvotes: i32,
    pub upvotes: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename(deserialize = "downloadURL"))]
    pub download_url: String,
    #[serde(rename(deserialize = "previewURL"))]
    pub preview_url: String,
    #[serde(rename(deserialize = "coverURL"))]
    pub cover_url: String,
    pub diffs: Vec<MapDifficulty>,
    pub hash: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapDifficulty {
    pub notes: i32,
    pub bombs: i32,
    pub characteristic: String,
    pub difficulty: String,
    pub njs: f32,
    pub nps: f32,
}

#[derive(Deserialize, Debug)]
pub struct Maps {
    pub docs: Vec<Map>,
}
