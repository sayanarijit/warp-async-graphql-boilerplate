use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;
use dotenv::dotenv;
use lazy_static::lazy_static;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use std::env;

use crate::models::*;

type DbConnection = ConnectionManager<PgConnection>;

fn new_poll() -> r2d2::Pool<DbConnection> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

lazy_static! {
    static ref PG_POOL: r2d2::Pool<DbConnection> = new_poll();
}

pub fn get_connection() -> Result<PooledConnection<DbConnection>, r2d2::Error> {
    PG_POOL.clone().get()
}

pub fn create_post<'a>(
    conn: &PgConnection,
    title: &'a str,
    body: &'a str,
) -> QueryResult<Post> {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}
