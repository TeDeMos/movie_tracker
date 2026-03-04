use {crate::tmdb::utils::maybe_date, chrono::NaiveDate, serde::Deserialize};

#[derive(Deserialize, Debug)]
pub struct Paginated<T> {
    page: usize,
    results: Vec<T>,
    total_pages: usize,
    total_results: usize,
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
pub struct Genre {
    id: i32,
    name: String,
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
    production_companies: Vec<ProductionCompany>,
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
    vote_count: f32,
    credits: MovieCredits,
}

#[derive(Deserialize, Debug)]
pub struct MovieCredits {
    cast: Vec<CastMember>,
    crew: Vec<CrewMember>,
}

#[derive(Deserialize, Debug)]
pub struct ProductionCompany {
    id: i32,
    logo_path: Option<String>,
    name: String,
    origin_country: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductionCountry {
    iso_3166_1: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchMovie {
    adult: bool,
    backdrop_path: Option<String>,
    genre_ids: Vec<i32>,
    id: i32,
    original_language: String,
    original_title: String,
    overview: String,
    popularity: f32,
    poster_path: Option<String>,
    #[serde(deserialize_with = "maybe_date")]
    release_date: Option<NaiveDate>,
    title: String,
    video: bool,
    vote_average: f32,
    vote_count: f32,
}

#[derive(Deserialize, Debug)]
pub struct SpokenLanguage {
    english_name: String,
    iso_639_1: String,
    name: String,
}
