use {
    crate::tmdb::{
        model::{Movie, Paginated, SearchMovie, SearchTv, TvEpisode, TvSeason, TvSeries},
        utils::{DebugJsonError, ResponseExt},
    },
    reqwest::blocking,
};

const TOKEN: &str = include_str!("../token.txt");
const LANGUAGE: (&str, &str) = ("language", "pl-PL");

pub struct Client(blocking::Client);

impl Client {
    pub fn new() -> Self { Self(blocking::Client::new()) }

    pub fn search_movie(
        &self, query: &str, page: i32,
    ) -> Result<Paginated<SearchMovie>, DebugJsonError> {
        self.0
            .get("https://api.themoviedb.org/3/search/movie")
            .query(&[("query", query), LANGUAGE, ("page", &page.to_string())])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn search_tv(&self, query: &str, page: i32) -> Result<Paginated<SearchTv>, DebugJsonError> {
        self.0
            .get("https://api.themoviedb.org/3/search/tv")
            .query(&[("query", query), LANGUAGE, ("page", &page.to_string())])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn movie(&self, movie_id: i32) -> Result<Movie, DebugJsonError> {
        self.0
            .get(format!("https://api.themoviedb.org/3/movie/{movie_id}"))
            .query(&[LANGUAGE, ("append_to_response", "credits,external_ids")])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn tv_series(&self, series_id: i32) -> Result<TvSeries, DebugJsonError> {
        self.0
            .get(format!("https://api.themoviedb.org/3/tv/{series_id}"))
            .query(&[LANGUAGE, ("append_to_response", "aggregate_credits,external_ids")])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn tv_season(
        &self, series_id: i32, season_number: i32,
    ) -> Result<TvSeason, DebugJsonError> {
        self.0
            .get(format!("https://api.themoviedb.org/3/tv/{series_id}/season/{season_number}"))
            .query(&[LANGUAGE])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn tv_episode(
        &self, series_id: i32, season_number: i32, episode_number: i32,
    ) -> Result<TvEpisode, DebugJsonError> {
        self.0
            .get(format!("https://api.themoviedb.org/3/tv/{series_id}/season/{season_number}/episode/{episode_number}"))
            .query(&[LANGUAGE, ("append_to_response", "credits,external_ids")])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }
}
