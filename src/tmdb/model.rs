use {crate::tmdb::utils::maybe_date, chrono::NaiveDate, serde::Deserialize};

#[derive(Deserialize, Debug)]
pub struct Paginated<T> {
    pub page: i32,
    pub results: Vec<T>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Deserialize, Debug)]
pub struct AggregateCastMember {
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
    roles: Vec<Role>,
    total_episode_count: i32,
    order: i32,
}

#[derive(Deserialize, Debug)]
pub struct AggregateCrewMember {
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
    jobs: Vec<Job>,
    department: String,
    total_episode_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct CastMember {
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
    cast_id: i32,
    character: String,
    credit_id: String,
    order: i32,
}

#[derive(Deserialize, Debug)]
pub struct CastMemberEpisode {
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
    character: String,
    credit_id: String,
    order: i32,
}

#[derive(Deserialize, Debug)]
pub struct Company {
    id: i32,
    logo_path: Option<String>,
    name: String,
    origin_country: String,
}

#[derive(Deserialize, Debug)]
pub struct CrewMember {
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
    credit_id: String,
    department: String,
    job: String,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeMid {
    #[serde(deserialize_with = "maybe_date")]
    air_date: Option<NaiveDate>,
    #[serde(rename = "episode_number")]
    number: i32,
    #[serde(rename = "episode_type")]
    r#type: String,
    id: i32,
    name: String,
    overview: String,
    production_code: String,
    runtime: i32,
    season_number: i32,
    show_id: i32,
    still_path: String,
    vote_average: f32,
    vote_count: i32,
    crew: Vec<CrewMember>,
    guest_stars: Vec<GuestStar>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeShort {
    id: i32,
    name: String,
    overview: String,
    vote_average: f32,
    vote_count: i32,
    #[serde(deserialize_with = "maybe_date")]
    air_date: Option<NaiveDate>,
    #[serde(rename = "episode_number")]
    number: i32,
    production_code: String,
    runtime: i32,
    season_number: i32,
    show_id: i32,
    still_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Genre {
    id: i32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct GuestStar {
    character: String,
    credit_id: String,
    order: i32,
    adult: bool,
    gender: i32,
    id: i32,
    known_for_department: String,
    name: String,
    original_name: String,
    popularity: f32,
    profile_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Job {
    credit_id: String,
    job: String,
    episode_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    adult: bool,
    backdrop_path: String,
    belongs_to_collection: Option<String>,
    budget: i32,
    genres: Vec<Genre>,
    homepage: String,
    id: i32,
    imdb_id: String,
    original_language: String,
    original_title: String,
    overview: String,
    popularity: f32,
    poster_path: String,
    production_companies: Vec<Company>,
    production_countries: Vec<ProductionCountry>,
    #[serde(deserialize_with = "maybe_date")]
    release_date: Option<NaiveDate>,
    revenue: i32,
    runtime: i32,
    spoken_languages: Vec<SpokenLanguage>,
    status: String,
    tagline: String,
    video: bool,
    vote_average: f32,
    vote_count: i32,
    credits: MovieCredits,
    external_ids: MovieExternalIds,
}

#[derive(Deserialize, Debug)]
pub struct MovieCredits {
    cast: Vec<CastMember>,
    crew: Vec<CrewMember>,
}

#[derive(Deserialize, Debug)]
pub struct MovieExternalIds {
    #[serde(rename = "imdb_id")]
    imdb: String,
    #[serde(rename = "wikidata_id")]
    wikidata: Option<String>,
    #[serde(rename = "facebook_id")]
    facebook: Option<String>,
    #[serde(rename = "instagram_id")]
    instagram: Option<String>,
    #[serde(rename = "twitter_id")]
    twitter: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProductionCountry {
    iso_3166_1: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Role {
    credit_id: String,
    character: String,
    episode_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct SearchMovie {
    adult: bool,
    backdrop_path: Option<String>,
    genre_ids: Vec<i32>,
    id: i32,
    original_language: String,
    pub original_title: String,
    overview: String,
    popularity: f32,
    poster_path: Option<String>,
    #[serde(deserialize_with = "maybe_date")]
    pub release_date: Option<NaiveDate>,
    title: String,
    video: bool,
    vote_average: f32,
    vote_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct SearchTv {
    adult: bool,
    backdrop_path: Option<String>,
    genre_ids: Vec<i32>,
    id: i32,
    origin_country: Vec<String>,
    original_language: String,
    original_name: String,
    overview: String,
    popularity: f32,
    poster_path: Option<String>,
    #[serde(deserialize_with = "maybe_date")]
    first_air_date: Option<NaiveDate>,
    name: String,
    vote_average: f32,
    vote_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct SpokenLanguage {
    english_name: String,
    iso_639_1: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct TvEpisode {
    #[serde(deserialize_with = "maybe_date")]
    air_date: Option<NaiveDate>,
    crew: Vec<CrewMember>,
    #[serde(rename = "episode_number")]
    number: i32,
    #[serde(rename = "episode_type")]
    r#type: String,
    guest_stars: Vec<GuestStar>,
    name: String,
    overview: String,
    id: i32,
    production_code: String,
    runtime: i32,
    season_number: i32,
    still_path: String,
    vote_average: f32,
    vote_count: i32,
    credits: TvEpisodeCredits,
    external_ids: TvEpisodeExternalIds,
}

#[derive(Deserialize, Debug)]
pub struct TvEpisodeCredits {
    cast: Vec<CastMemberEpisode>,
    crew: Vec<CrewMember>,
    guest_stars: Vec<GuestStar>,
}

#[derive(Deserialize, Debug)]
pub struct TvEpisodeExternalIds {
    #[serde(rename = "imdb_id")]
    imdb: String,
    freebase_mid: String,
    #[serde(rename = "freebase_id")]
    freebase: Option<String>,
    #[serde(rename = "tvdb_id")]
    tvdb: i32,
    #[serde(rename = "tvrage_id")]
    tvrage: i32,
    #[serde(rename = "wikidata_id")]
    wikidata: String,
}

#[derive(Deserialize, Debug)]
pub struct TvSeason {
    _id: String,
    #[serde(deserialize_with = "maybe_date")]
    air_date: Option<NaiveDate>,
    episodes: Vec<EpisodeMid>,
    name: String,
    networks: Vec<Company>,
    overview: String,
    id: i32,
    poster_path: Option<String>,
    season_number: i32,
    vote_average: f32,
}

#[derive(Deserialize, Debug)]
pub struct TvSeries {
    adult: bool,
    backdrop_path: Option<String>,
    created_by: Vec<SeriesCreator>,
    episode_run_time: Vec<i32>,
    #[serde(deserialize_with = "maybe_date")]
    first_air_date: Option<NaiveDate>,
    genres: Vec<Genre>,
    homepage: String,
    id: i32,
    in_production: bool,
    languages: Vec<String>,
    #[serde(deserialize_with = "maybe_date")]
    last_air_date: Option<NaiveDate>,
    last_episode_to_air: Option<EpisodeShort>,
    name: String,
    #[serde(deserialize_with = "maybe_date")]
    next_episode_to_air: Option<NaiveDate>,
    networks: Vec<Company>,
    number_of_episodes: i32,
    number_of_seasons: i32,
    origin_country: Vec<String>,
    original_language: String,
    original_name: String,
    overview: String,
    popularity: f32,
    poster_path: Option<String>,
    production_companies: Vec<Company>,
    production_countries: Vec<ProductionCountry>,
    seasons: Vec<SeasonShort>,
    spoken_languages: Vec<SpokenLanguage>,
    status: String,
    tagline: String,
    r#type: String,
    vote_average: f32,
    vote_count: i32,
    aggregate_credits: TvSeriesAggregateCredits,
    external_ids: TvSeriesExternalIds,
}

#[derive(Deserialize, Debug)]
pub struct TvSeriesAggregateCredits {
    cast: Vec<AggregateCastMember>,
    crew: Vec<AggregateCrewMember>,
}

#[derive(Deserialize, Debug)]
pub struct TvSeriesExternalIds {
    #[serde(rename = "imdb_id")]
    imdb: String,
    freebase_mid: String,
    #[serde(rename = "freebase_id")]
    freebase: String,
    #[serde(rename = "tvdb_id")]
    tvdb: i32,
    #[serde(rename = "tvrage_id")]
    tvrage: i32,
    #[serde(rename = "wikidata_id")]
    wikidata: String,
    #[serde(rename = "facebook_id")]
    facebook: String,
    #[serde(rename = "instagram_id")]
    instagram: String,
    #[serde(rename = "twitter_id")]
    twitter: String,
}

#[derive(Deserialize, Debug)]
pub struct SeasonShort {
    #[serde(deserialize_with = "maybe_date")]
    air_date: Option<NaiveDate>,
    episode_count: i32,
    id: i32,
    name: String,
    overview: String,
    poster_path: Option<String>,
    #[serde(rename = "season_number")]
    number: i32,
    vote_average: f32,
}

#[derive(Deserialize, Debug)]
pub struct SeriesCreator {
    id: i32,
    credit_id: String,
    name: String,
    gender: i32,
    profile_path: String,
}
