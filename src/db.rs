use {
    crate::{
        db::{
            model::{Cast, Crew, Movie, MovieDetails, Person},
            utils::TryJoin,
        },
        tmdb::model as api,
    },
    chrono::NaiveDate,
    directories::ProjectDirs,
    rusqlite::{Connection, OptionalExtension, Result},
    std::{borrow::Cow, fs},
};

pub mod model;
mod utils;

pub struct Database(Connection);

impl Database {
    pub fn new() -> Result<Self> {
        let dirs = ProjectDirs::from("com", "tedemos", "movie_tracker").unwrap();
        let data_path = dirs.data_local_dir();
        fs::create_dir_all(data_path).unwrap();
        let connection = Connection::open(data_path.join("db.sqlite"))?;
        connection.execute_batch(include_str!("db/schema.sql"))?;
        Ok(Self(connection))
    }

    pub fn get_movie_details(&self, id: i32) -> Result<Option<MovieDetails>> {
        let Some(movie) =
            self.0.prepare(Movie::SELECT_ID)?.query_one([id], |r| Movie::try_from(r)).optional()?
        else {
            return Ok(None);
        };
        let cast = self
            .0
            .prepare(
                "select p.name from people p inner join cast c on p.id = c.person_id where \
                 c.movie_id = ?1 order by c.credit_order limit 3",
            )?
            .query_map([id], |r| r.get(0))?
            .try_join(", ")?;
        let directors = self
            .0
            .prepare(
                "select p.name from people p inner join crew c on p.id = c.person_id where \
                 c.movie_id = ?1 and c.job = 'Director' limit 3",
            )?
            .query_map([id], |r| r.get(0))?
            .try_join(", ")?;
        let watched: (u32, Option<NaiveDate>) = self
            .0
            .prepare("select count(*), max(viewing_date) from viewings where movie_id = ?1")?
            .query_one([id], |r| r.try_into())?;
        Ok(Some(MovieDetails {
            imdb_url: format!("https://www.imdb.com/title/{}", movie.imdb_id),
            titles: if movie.original_title == movie.title {
                movie.title.into_owned()
            } else {
                format!("{} ({})", movie.title, movie.original_title)
            },
            language: movie.language.into_owned(),
            runtime: if movie.runtime < 60 {
                format!("{}m", movie.runtime)
            } else {
                format!("{}h {}m", movie.runtime / 60, movie.runtime % 60)
            },
            release_date: match movie.release_date {
                Some(d) => d.format("%Y-%m-%d").to_string(),
                None => "Unknown".into(),
            },
            overview: match movie.overview {
                Some(o) => o.into_owned(),
                None => "No overview".into(),
            },
            cast,
            directors,
            previously_watched: match watched {
                (_, None) => "No".into(),
                (1, Some(d)) => format!("On {}", d.format("%Y-%m-%d")),
                (c, Some(d)) => format!("{c} times, last one on {}", d.format("%Y-%m-%d")),
            },
        }))
    }

    pub fn insert_movie(&mut self, api_movie: &api::Movie) -> Result<()> {
        let tx = self.0.transaction()?;

        let movie = Movie::from_api_movie(api_movie);
        tx.execute(Movie::INSERT, movie.params())?;

        {
            let mut add_person = tx.prepare(Person::UPSERT)?;
            let mut add_cast = tx.prepare(Cast::INSERT)?;
            let mut crew_insert = tx.prepare(Crew::UPSERT)?;

            for c in &api_movie.credits.cast {
                add_person.execute(Person::from_api_cast_member(c).params())?;
                add_cast.execute(Cast::from_api_movie_and_cast_member(api_movie, c).params())?;
            }

            for c in &api_movie.credits.crew {
                let Some(crew) = Crew::from_api_movie_and_crew_member(api_movie, c) else {
                    continue;
                };
                add_person.execute(Person::from_api_crew_member(c).params())?;
                crew_insert.execute(crew.params())?;
            }
        }

        tx.commit()
    }
}

// - execute - 1 statement, no results
// - execute_batch - many statements
// - prepare - one statement, reused many times
//   - execute - same as connection
//   - exists - whether statement returns 1 or more rows
//   - insert - insert, returns row id
//   - query - iterator over resulting rows (next, map, mapped, and_then)
//   - query_and_then - iterator over resulting rows with a fallible mapping function allowing any
//     error type
//   - query_map - iterator over resulting rows with a fallible mapping function with a forced error
//     type
//   - query_one - same as connection
//   - query_row - same as connection
// - transaction - begin a transaction - derefs to connection, allows for rollbacks, one big commit
// - query_one - one statement, one row result, fallible mapping function with a forced result type
// - query_row - one statement, rows after the first ignored, fallible mapping function with a
//   forced result type
// - query_row_and_then - one statement, rows after the first ignored, fallible mapping
// - function allowing any error type
// - optional - wrapping around the errors, detecting QueryReturnedNoRows and turning it into
//   Ok(None)
