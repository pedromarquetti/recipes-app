pub mod recipe;
pub mod user;

use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as R2D2Pool},
};
use serde_derive::{Deserialize, Serialize};
use warp::Rejection;

use crate::error::convert_to_rejection;

pub type DieselError = diesel::result::Error;
pub type R2D2Err = r2d2::Error;
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = Result<PooledConnection<ConnectionManager<PgConnection>>, R2D2Err>;
pub type OkPool = PooledConnection<ConnectionManager<PgConnection>>;

pub fn connect_to_db(url: String) -> Result<Pool, Rejection> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .map_err(convert_to_rejection)
}
