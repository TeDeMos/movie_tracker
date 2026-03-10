use {
    crate::tmdb::model::CrewMember,
    chrono::NaiveDate,
    rusqlite::{
        Result, Row, ToSql,
        types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    },
    strum::{Display, EnumString},
};

pub struct Movie {
    pub id: i32,
    pub imdb_id: String,
    pub language: String,
    pub title: String,
    pub overview: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub runtime: i32,
}

pub struct Person {
    id: i32,
    name: String,
}

pub struct CastEntry {
    movie_id: i32,
    person_id: i32,
    character: String,
    credit_order: i32,
}

pub struct CrewEntry {
    movie_id: i32,
    person_id: i32,
    job: Job,
}

#[derive(EnumString, Display)]
pub enum Job {
    Producer,
    Director,
    Writer,
}

impl Job {
    pub fn from_crew_member(member: &CrewMember) -> Option<Self> {
        match (member.department.as_str(), member.job.as_str()) {
            ("Production", "Executive Producer" | "Producer" | "Co-Producer") =>
                Some(Job::Producer),
            ("Directing", "Series Director" | "Director" | "Co-Director") => Some(Job::Director),
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
            ) => Some(Job::Writer),
            _ => None,
        }
    }
}

impl FromSql for Job {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str()?.try_into().map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for Job {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::from(self.to_string())) }
}

impl Movie {
    pub fn try_from_row(row: &Row) -> Result<Self> {
        let (id, imdb_id, language, title, overview, release_date, runtime) = row.try_into()?;
        Ok(Self { id, imdb_id, language, title, overview, release_date, runtime })
    }
}

impl Person {
    fn try_from_row(row: &Row) -> Result<Self> {
        let (id, name) = row.try_into()?;
        Ok(Self { id, name })
    }
}

impl CastEntry {
    fn try_from_row(row: &Row) -> Result<Self> {
        let (movie_id, person_id, character, credit_order) = row.try_into()?;
        Ok(Self { movie_id, person_id, character, credit_order })
    }
}

impl CrewEntry {
    fn try_from_row(row: &Row) -> Result<Self> {
        let (movie_id, person_id, job) = row.try_into()?;
        Ok(Self { movie_id, person_id, job })
    }
}
