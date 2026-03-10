mod model;

use {
    crate::{
        db::model::{Job, Movie},
        tmdb::model::Movie as ApiMovie,
    },
    directories::ProjectDirs,
    rusqlite::{Connection, OptionalExtension, Result},
    std::fs,
};

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

    pub fn get_movie(&self, id: i32) -> Result<Option<Movie>> {
        self.0.query_one("select * from movies where id = ?1", [id], Movie::try_from_row).optional()
    }

    pub fn insert_movie(&mut self, api_movie: ApiMovie) -> Result<()> {
        let tx = self.0.transaction()?;

        tx.execute(
            "insert into movies (id, imdb_id, language, title, overview, release_date, runtime) \
             values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                api_movie.id,
                api_movie.imdb_id,
                api_movie.original_language,
                api_movie.original_title,
                (!api_movie.overview.is_empty()).then_some(api_movie.overview),
                api_movie.release_date,
                api_movie.runtime,
            ),
        )?;

        let mut people_insert = tx
            .prepare("insert into people (id, name) values (?1, ?2) on conflict (id) do nothing")?;
        let mut cast_insert = tx.prepare(
            "insert into cast (movie_id, person_id, character, credit_order) values (?1, ?2, ?3, \
             ?4)",
        )?;
        let mut crew_insert = tx.prepare(
            "insert into crew (movie_id, person_id, job) values (?1, ?2, ?3) on conflict \
             (movie_id, person_id, job) do nothing",
        )?;

        for c in &api_movie.credits.cast {
            people_insert.execute((c.id, &c.name))?;
            cast_insert.execute((api_movie.id, c.id, &c.character, c.order))?;
        }

        for c in &api_movie.credits.crew {
            let Some(job) = Job::from_crew_member(c) else { continue };
            println!("{}, {}, {job}", c.name, c.job);
            people_insert.execute((c.id, &c.name))?;
            crew_insert.execute((api_movie.id, c.id, job))?;
        }
        people_insert.finalize()?;
        cast_insert.finalize()?;
        crew_insert.finalize()?;
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
