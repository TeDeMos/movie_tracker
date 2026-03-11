use {
    crate::tmdb::model as api,
    chrono::NaiveDate,
    rusqlite::{Params, ToSql, types::ToSqlOutput},
};

#[derive(Copy, Clone)]
enum MediaId {
    Movie(i32),
    Episode(i32),
}

impl MediaId {
    fn as_ids(self) -> (Option<i32>, Option<i32>) {
        match self {
            MediaId::Movie(id) => (Some(id), None),
            MediaId::Episode(id) => (None, Some(id)),
        }
    }

    fn from_api_movie(value: &api::Movie) -> Self { Self::Movie(value.id) }
}

struct Viewing {
    media_id: MediaId,
    date: NaiveDate,
    rating: f32,
}

impl Viewing {
    const INSERT: &str = "insert into viewings (movie_id, episode_id, viewing_date, order_in_day, \
                          rating) values (?1, ?2, ?3, (select coalesce(max(order_in_day) + 1, 0) \
                          from viewings where viewing_date = ?3), ?4)";

    fn params(&self) -> impl Params {
        let (movie_id, episode_id) = self.media_id.as_ids();
        (movie_id, episode_id, self.date, self.rating)
    }
}

struct Tag<'a> {
    name: &'a str,
}

impl Tag<'_> {
    const INSERT: &'static str = "insert into tags (name) values (?1)";

    fn params(&self) -> impl Params { [&self.name] }
}

struct ViewingTag {
    viewing_id: i32,
    tag_id: i32,
}

impl ViewingTag {
    const INSERT: &str = "insert into viewing_tags (viewing_id, tag_id) values (?1, ?2)";

    fn params(&self) -> impl Params { (self.viewing_id, self.tag_id) }
}

pub struct Movie<'a> {
    id: i32,
    imdb_id: &'a str,
    original_title: &'a str,
    title: &'a str,
    language: &'a str,
    runtime: u32,
    release_date: Option<NaiveDate>,
    overview: Option<&'a str>,
}

impl<'a> Movie<'a> {
    pub const INSERT: &'static str = "insert into movies (id, imdb_id, original_title, title, \
                                      language, runtime, release_date, overview) values (?1, ?2, \
                                      ?3, ?4, ?5, ?6, ?7, ?8)";

    pub fn params(&self) -> impl Params {
        (
            self.id, self.imdb_id, self.original_title, self.title, self.language, self.runtime,
            self.release_date, self.overview,
        )
    }

    pub fn from_api_movie(value: &'a api::Movie) -> Self {
        Self {
            id: value.id,
            imdb_id: &value.imdb_id,
            original_title: &value.original_title,
            title: &value.title,
            language: &value.original_language,
            runtime: value.runtime,
            release_date: value.release_date,
            overview: (!value.overview.is_empty()).then_some(&value.overview),
        }
    }
}

struct Series<'a> {
    id: i32,
    imdb_id: &'a str,
    original_name: &'a str,
    name: &'a str,
    language: &'a str,
    number_of_seasons: u32,
    number_of_episodes: u32,
    first_air_date: Option<NaiveDate>,
    last_air_date: Option<NaiveDate>,
    in_production: bool,
    overview: Option<&'a str>,
}

impl Series<'_> {
    const INSERT: &'static str = "insert into series (id, imdb_id, original_name, name, language, \
                                  number_of_seasons, number_of_episodes, first_air_date, \
                                  last_air_date, in_production, overview) values (?1, ?2, ?3, ?4, \
                                  ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

    fn params(&self) -> impl Params {
        (
            self.id, self.imdb_id, self.original_name, self.name, self.language,
            self.number_of_seasons, self.number_of_episodes, self.first_air_date,
            self.last_air_date, self.in_production, self.overview,
        )
    }
}

struct Season<'a> {
    id: i32,
    series_id: i32,
    number: u32,
    name: &'a str,
    air_date: Option<NaiveDate>,
    number_of_episodes: u32,
    overview: Option<&'a str>,
}

impl Season<'_> {
    const INSERT: &'static str = "insert into seasons (id, series_id, number, name, air_date, \
                                  number_of_episodes, overview) values (?1, ?2, ?3, ?4, ?5, ?6, \
                                  ?7)";

    fn params(&self) -> impl Params {
        (
            self.id, self.series_id, self.number, self.name, self.air_date,
            self.number_of_episodes, self.overview,
        )
    }
}

struct Episode<'a> {
    id: i32,
    imdb_id: &'a str,
    season_id: i32,
    number: u32,
    name: &'a str,
    air_date: Option<NaiveDate>,
    runtime: u32,
    overview: Option<&'a str>,
}

impl Episode<'_> {
    const INSERT: &'static str = "insert into episodes (id, imdb_id, season_id, number, name, \
                                  air_date, runtime, overview) values (?1, ?2, ?3, ?4, ?5, ?6, \
                                  ?7, ?8)";

    fn params(&self) -> impl Params {
        (
            self.id, self.imdb_id, self.season_id, self.number, self.name, self.air_date,
            self.runtime, self.overview,
        )
    }
}

#[derive(Copy, Clone)]
enum Gender {
    Unknown,
    Male,
    Female,
    NonBinary,
}

impl Gender {
    fn from_api_gender(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Female,
            2 => Self::Male,
            3 => Self::NonBinary,
            _ => unreachable!(),
        }
    }
}

impl ToSql for Gender {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            Self::Unknown => "U",
            Self::Male => "M",
            Self::Female => "F",
            Self::NonBinary => "N",
        }))
    }
}

pub struct Person<'a> {
    id: i32,
    name: &'a str,
    gender: Gender,
}

impl<'a> Person<'a> {
    const INSERT: &'static str = "insert into people (id, name, sex) values (?1, ?2, ?3)";
    pub const UPSERT: &'static str =
        "insert into people (id, name, gender) values (?1, ?2, ?3) on conflict (id) do nothing";

    pub fn params(&self) -> impl Params { (self.id, self.name, self.gender) }

    pub fn from_api_cast_member(value: &'a api::CastMember) -> Self {
        Self { id: value.id, name: &value.name, gender: Gender::from_api_gender(value.gender) }
    }

    pub fn from_api_crew_member(value: &'a api::CrewMember) -> Self {
        Self { id: value.id, name: &value.name, gender: Gender::from_api_gender(value.gender) }
    }
}

pub struct Cast<'a> {
    media_id: MediaId,
    person_id: i32,
    character: &'a str,
    credit_order: u32,
}

impl<'a> Cast<'a> {
    pub const INSERT: &'static str = "insert into cast (movie_id, episode_id, person_id, \
                                      character, credit_order) values (?1, ?2, ?3, ?4, ?5)";

    pub fn params(&self) -> impl Params {
        let (movie_id, episode_id) = self.media_id.as_ids();
        (movie_id, episode_id, self.person_id, self.character, self.credit_order)
    }

    pub fn from_api_movie_and_cast_member(
        movie: &api::Movie, cast_member: &'a api::CastMember,
    ) -> Self {
        Self {
            media_id: MediaId::from_api_movie(movie),
            person_id: cast_member.id,
            character: &cast_member.character,
            credit_order: cast_member.order,
        }
    }
}

#[derive(Copy, Clone)]
enum Job {
    Producer,
    Director,
    Writer,
}

impl Job {
    fn from_api_crew_member(value: &api::CrewMember) -> Option<Self> {
        match (value.department.as_str(), value.job.as_str()) {
            ("Production", "Executive Producer" | "Producer" | "Co-Producer") =>
                Some(Self::Producer),
            ("Directing", "Series Director" | "Director" | "Co-Director") => Some(Self::Director),
            (
                "Writing",
                "Screenplay"
                | "Teleplay"
                | "Writer"
                | "Co-Writer"
                | "Story"
                | "Screenstory"
                | "Author"
                | "Original Series Creator",
            ) => Some(Self::Writer),
            _ => None,
        }
    }
}

impl ToSql for Job {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            Job::Producer => "Producer",
            Job::Director => "Director",
            Job::Writer => "Writer",
        }))
    }
}

pub struct Crew {
    media_id: MediaId,
    person_id: i32,
    job: Job,
}

impl Crew {
    const INSERT: &str =
        "insert into crew (movie_id, episode_id, person_id, job) values (?1, ?2, ?3, ?4)";
    pub const UPSERT: &str = "insert into crew (movie_id, episode_id, person_id, job) values (?1, \
                              ?2, ?3, ?4) on conflict (movie_id, episode_id, person_id, job) do \
                              nothing";

    pub fn params(&self) -> impl Params {
        let (movie_id, episode_id) = self.media_id.as_ids();
        (movie_id, episode_id, self.person_id, self.job)
    }

    pub fn from_api_movie_and_crew_member(
        movie: &api::Movie, crew_member: &api::CrewMember,
    ) -> Option<Self> {
        Some(Self {
            media_id: MediaId::from_api_movie(movie),
            person_id: crew_member.id,
            job: Job::from_api_crew_member(crew_member)?,
        })
    }
}

struct SeriesCreator {
    series_id: i32,
    person_id: i32,
}

impl SeriesCreator {
    const INSERT: &str = "insert into series_creators (series_id, person_id) values (?1, ?2)";

    fn params(&self) -> impl Params { (self.series_id, self.person_id) }
}
