
pub mod models;
mod schema;

use chrono::offset::Utc;
use diesel::pg::PgConnection;
use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use diesel::r2d2::{
    ConnectionManager,
    Pool,
    PooledConnection
};
use diesel;
use self::models::*;
use self::schema::*;
use std::env;
use std::ops::Deref;

/// While the struct itself and the connection are public, Database cannot be manually
/// instantiated. Use Database::connect() to start it.
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    _hidden: (),
}

impl Database {
    /// Create a new database with a connection.
    /// Returns a new Database.
    pub fn connect() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to make connection pool");

        Database {
            pool,
            _hidden: (),
        }
    }

    /// Request a connection from the connection pool
    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.clone().get().expect("Attempt to get connection timed out")
    }

    // Guild Tools
    /// Add a guild with a given ID.
    /// Returns the Ok(Some(Guild)) on success or Ok(None) if there is a conflict.
    /// May return Err(DatabaseError) in the event of some other failure.
    pub fn new_guild(&self, id: i64) -> QueryResult<Option<Guild>> {
        let guild = NewGuild {
            id,
        };
        diesel::insert_into(guilds::table)
            .values(&guild)
            .on_conflict_do_nothing()
            .get_result(self.conn().deref())
            .optional()
    }
    /// Add multiple guilds with a vector of IDs
    /// Does nothing on conflict
    /// Returns Result<count, err>
    pub fn new_guilds(&self, ids: &[i64]) -> QueryResult<usize> {
        let guilds = {
            ids.iter().map(|e| {
                NewGuild {
                    id: *e,
                }
            }).collect::<Vec<NewGuild>>()
        };
        diesel::insert_into(guilds::table)
            .values(&guilds)
            .on_conflict_do_nothing()
            .execute(self.conn().deref())
    }
    /// Delete a guild by the ID.
    /// Returns Result<guild_id, err>
    pub fn del_guild(&self, g_id: i64) -> QueryResult<i64> {
        use crate::db::schema::guilds::columns::id;
        diesel::delete(guilds::table)
            .filter(id.eq(&g_id))
            .returning(id)
            .get_result(self.conn().deref())
    }
    /// Select a guild
    /// Returns Result<Guild, Err>
    pub fn get_guild(&self, g_id: i64) -> QueryResult<Guild> {
        guilds::table.find(&g_id)
            .first(self.conn().deref())
    }
    /// Update a guild
    /// Returns Result<Guild, Err>
    pub fn update_guild(&self, g_id: i64, guild: Guild) -> QueryResult<Guild> {
        let target = guilds::table.find(&g_id);
        diesel::update(target)
            .set(&guild)
            .get_result(self.conn().deref())
    }
    /// Get the count of guilds in the database
    pub fn count_guilds(&self) -> QueryResult<i64> {
        use diesel::dsl::count_star;
        guilds::table.select(count_star())
            .get_result(self.conn().deref())
    }
}
