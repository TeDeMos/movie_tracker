use {
    crate::tmdb::{
        model::{Movie, Paginated, SearchMovie, SearchTv, TvEpisode, TvSeason, TvSeries},
        utils::{DebugJsonError, ResponseExt},
    },
    derive_more::TryInto,
    reqwest::blocking::Client,
    serde::de::DeserializeOwned,
    std::{
        result,
        sync::mpsc::{self, Receiver, Sender, TryRecvError},
        thread,
    },
};

const TOKEN: &str = include_str!("../token.txt");
const LANGUAGE: (&str, &str) = ("language", "pl-PL");

enum Request {
    SearchMovie(SearchArgs),
    SearchTv(SearchArgs),
    Movie(i32),
    TvSeries(i32),
    TvSeason(SeasonArgs),
    TvEpisode(EpisodeArgs),
}

struct SearchArgs {
    query: String,
    page: i32,
}

struct SeasonArgs {
    series_id: i32,
    season_number: i32,
}

#[derive(Hash, Eq, PartialEq)]
struct EpisodeArgs {
    series_id: i32,
    season_number: i32,
    episode_number: i32,
}

struct RequestWrapper {
    id: usize,
    request: Request,
}

type Result<T> = result::Result<T, DebugJsonError>;

#[expect(clippy::large_enum_variant)]
#[derive(TryInto)]
enum Response {
    SearchMovie(Result<Paginated<SearchMovie>>),
    SearchTv(Result<Paginated<SearchTv>>),
    Movie(Result<Movie>),
    TvSeries(Result<TvSeries>),
    TvSeason(Result<TvSeason>),
    TvEpisode(Result<TvEpisode>),
}

struct ResponseWrapper {
    id: usize,
    response: Response,
}

pub struct TmdbClient {
    request_sender: Sender<RequestWrapper>,
    response_receiver: Receiver<ResponseWrapper>,
    next_id: usize,
}

impl TmdbClient {
    pub fn new() -> Self {
        let (request_sender, request_receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        let next_id = 0;
        thread::spawn(move || worker_thread(&request_receiver, &response_sender));
        Self { request_sender, response_receiver, next_id }
    }

    pub fn search_movie(&mut self, query: String, page: i32) -> usize {
        self.send_request(Request::SearchMovie(SearchArgs { query, page }))
    }

    pub fn search_tv(&mut self, query: String, page: i32) -> usize {
        self.send_request(Request::SearchTv(SearchArgs { query, page }))
    }

    pub fn movie(&mut self, movie_id: i32) -> usize { self.send_request(Request::Movie(movie_id)) }

    pub fn tv_series(&mut self, movie_id: i32) -> usize {
        self.send_request(Request::TvSeries(movie_id))
    }

    pub fn tv_season(&mut self, series_id: i32, season_number: i32) -> usize {
        self.send_request(Request::TvSeason(SeasonArgs { series_id, season_number }))
    }

    pub fn tv_episode(&mut self, series_id: i32, season_number: i32, episode_number: i32) -> usize {
        self.send_request(Request::TvEpisode(EpisodeArgs {
            series_id,
            season_number,
            episode_number,
        }))
    }

    fn send_request(&mut self, request: Request) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let wrapper = RequestWrapper { id, request };
        self.request_sender.send(wrapper).unwrap();
        id
    }

    pub fn search_movie_results(&mut self, id: usize) -> Option<Result<Paginated<SearchMovie>>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    pub fn search_tv_results(&mut self, id: usize) -> Option<Result<Paginated<SearchTv>>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    pub fn movie_results(&mut self, id: usize) -> Option<Result<Movie>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    pub fn tv_series_results(&mut self, id: usize) -> Option<Result<TvSeries>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    pub fn tv_season_results(&mut self, id: usize) -> Option<Result<TvSeason>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    pub fn tv_episode_results(&mut self, id: usize) -> Option<Result<TvEpisode>> {
        self.receive_response(id).and_then(|r| r.try_into().ok())
    }

    fn receive_response(&self, id: usize) -> Option<Response> {
        loop {
            match self.response_receiver.try_recv() {
                Ok(wrapper) =>
                    if wrapper.id == id {
                        return Some(wrapper.response);
                    },
                Err(TryRecvError::Empty) => return None,
                Err(TryRecvError::Disconnected) => panic!("Worker thread disconnected"),
            }
        }
    }
}

fn worker_thread(
    request_receiver: &Receiver<RequestWrapper>, response_sender: &Sender<ResponseWrapper>,
) {
    let client = Client::new();
    while let Ok(request_wrapper) = request_receiver.recv() {
        let response = match request_wrapper.request {
            Request::SearchMovie(args) => Response::SearchMovie(make_request(
                &client,
                "https://api.themoviedb.org/3/search/movie",
                &[("query", &args.query), LANGUAGE, ("page", &args.page.to_string())],
            )),
            Request::SearchTv(args) => Response::SearchTv(make_request(
                &client,
                "https://api.themoviedb.org/3/search/tv",
                &[("query", &args.query), LANGUAGE, ("page", &args.page.to_string())],
            )),
            Request::Movie(movie_id) => Response::Movie(make_request(
                &client,
                &format!("https://api.themoviedb.org/3/movie/{movie_id}"),
                &[LANGUAGE, ("append_to_response", "credits,external_ids")],
            )),
            Request::TvSeries(series_id) => Response::TvSeries(make_request(
                &client,
                &format!("https://api.themoviedb.org/3/tv/{series_id}"),
                &[LANGUAGE, ("append_to_response", "aggregate_credits,external_ids")],
            )),
            Request::TvSeason(args) => Response::TvSeason(make_request(
                &client,
                &format!(
                    "https://api.themoviedb.org/3/tv/{}/season/{}",
                    args.series_id, args.season_number
                ),
                &[LANGUAGE],
            )),
            Request::TvEpisode(args) => Response::TvEpisode(make_request(
                &client,
                &format!(
                    "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}",
                    args.series_id, args.season_number, args.episode_number
                ),
                &[LANGUAGE, ("append_to_response", "credits,external_ids")],
            )),
        };
        let response_wrapper = ResponseWrapper { id: request_wrapper.id, response };
        response_sender.send(response_wrapper).unwrap();
    }
}

fn make_request<T: DeserializeOwned>(
    client: &Client, url: &str, query: &[(&str, &str)],
) -> Result<T> {
    client
        .get(url)
        .query(query)
        .bearer_auth(TOKEN)
        .header("accept", "application/json")
        .send()?
        .debug_json()
}
