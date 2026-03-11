mod model;

use {
    crate::{
        db::model::{Cast, Crew, Movie, Person},
        tmdb::model as api,
    },
    directories::ProjectDirs,
    rusqlite::{Connection, Result},
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

    // pub fn get_movie(&self, id: i32) -> Result<Option<Movie>> {
    //     self.0.query_one("select * from movies where id = ?1", [id],
    // Movie::try_from_row).optional() }

    pub fn movie_exists(&self, id: i32) -> Result<bool> {
        self.0.prepare("select 1 from movies where id = ?1")?.exists([id])
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
