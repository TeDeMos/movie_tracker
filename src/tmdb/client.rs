use {
    crate::tmdb::{
        model::{Movie, Paginated, SearchMovie},
        utils::{DebugJsonError, ResponseExt},
    },
    reqwest::blocking,
};

const TOKEN: &str = include_str!("../token.txt");

pub struct Client(blocking::Client);

impl Client {
    pub fn new() -> Self { Self(blocking::Client::new()) }

    pub fn search_movie(
        &self, query: &str, page: usize,
    ) -> Result<Paginated<SearchMovie>, DebugJsonError> {
        self.0
            .get("https://api.themoviedb.org/3/search/movie")
            .query(&[("query", query), ("language", "pl-PL"), ("page", &page.to_string())])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }

    pub fn movie(&self, movie_id: i32) -> Result<Movie, DebugJsonError> {
        self.0
            .get(format!("https://api.themoviedb.org/3/movie/{movie_id}"))
            .query(&[("language", "pl-PL"), ("append_to_response", "credits")])
            .bearer_auth(TOKEN)
            .header("accept", "application/json")
            .send()?
            .debug_json()
    }
}
