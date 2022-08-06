use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Show {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub seasons: Vec<ShowSeason>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShowSeason {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub season_number: u32,

    pub episode_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub season_number: u32,

    pub episodes: Vec<SeasonEpisode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonEpisode {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub season_number: u32,
    pub episode_number: u32,
    pub air_date: NaiveDate,
}
