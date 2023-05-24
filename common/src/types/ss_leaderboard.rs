use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaderBoardInfo {
    pub id: i32,
    pub max_score: i32,
    pub stars: f32,
    pub difficulties: Vec<Difficulty>,
    pub difficulty: Difficulty,
}

impl LeaderBoardInfo {
    pub fn new() -> LeaderBoardInfo {
        LeaderBoardInfo {
            id: 0,
            max_score: 0,
            stars: 0.0,
            difficulties: Vec::new(),
            difficulty: Difficulty {
                leaderboard_id: 0,
                difficulty: 0,
                game_mode: String::new(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub leaderboard_id: u32,
    pub difficulty: u8,
    pub game_mode: String,
}

#[derive(Deserialize, Debug)]
pub struct Leaderboard {
    pub scores: Vec<Score>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: i32,
    pub leaderboard_player_info: Player,
    pub rank: i32,
    pub base_score: i32,
    pub pp: f32,
    pub bad_cuts: i32,
    pub missed_notes: i32,
    pub max_combo: i32,
    pub full_combo: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub country: String,
}
