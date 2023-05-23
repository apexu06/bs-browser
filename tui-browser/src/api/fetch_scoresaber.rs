use crate::types::leaderboard::{LeaderBoardInfo, Leaderboard, Score};

pub async fn fetch_leaderboard(
    leaderboard_id: u32,
    page: u32,
) -> Result<Vec<Score>, Box<dyn std::error::Error>> {
    let resp: Leaderboard = reqwest::get(&format!(
        "https://scoresaber.com/api/leaderboard/by-id/{}/scores?page={}",
        leaderboard_id, page
    ))
    .await?
    .json()
    .await?;

    Ok(resp.scores)
}

pub async fn fetch_leaderboard_info(
    song_hash: &str,
    difficulty: u8,
    game_mode: &str,
) -> Result<LeaderBoardInfo, Box<dyn std::error::Error>> {
    let resp: LeaderBoardInfo = reqwest::get(&format!(
        "https://scoresaber.com/api/leaderboard/by-hash/{}/info?difficulty={}&gameMode={}",
        song_hash, difficulty, game_mode
    ))
    .await?
    .json()
    .await?;

    Ok(resp)
}
