use std::fmt::Display;
use std::time::Duration;

use diesel::r2d2::PooledConnection;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as R2D2Pool},
};

pub type DieselError = diesel::result::Error;
pub type R2D2Err = r2d2::Error;
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = Result<PooledConnection<ConnectionManager<PgConnection>>, R2D2Err>;
pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn connect_to_db<S: Display>(url: S) -> Result<Pool, R2D2Err> {
    let manager = ConnectionManager::<PgConnection>::new(url.to_string());
    Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
}
