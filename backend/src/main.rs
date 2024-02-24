use db::db_pool::{connect_to_db, Pool};
use dotenvy::dotenv;
use error::handle_rejection;
use log::info;
use routes::routing_table;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use warp::{Filter, Rejection};

use crate::error::convert_to_rejection;

mod error;
mod jwt;
mod routes;

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

fn get_db_url() -> String {
    env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL))
}

#[tokio::main]
async fn main() -> Result<(), Rejection> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "backend")
    }
    env_logger::init();
    dotenv().ok();
    let db_pool: Pool = connect_to_db(get_db_url()).map_err(convert_to_rejection)?;

    let routes = routing_table(db_pool)
        .recover(handle_rejection)
        .boxed()
        .and(warp::addr::remote())
        .map(|routes, address: Option<SocketAddr>| {
            info!("Request from: {:?} handled!", address.unwrap());
            return routes;
        });

    // address used by the server
    let ip: String = env::var("SERVER_IP").unwrap_or("0.0.0.0".into());
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".into());

    let address = SocketAddr::new(
        IpAddr::from_str(&ip).expect("expected valid IP address"),
        port.parse::<u16>().expect("expected valid SERVER_PORT"),
    );
    info!("running server at {} ", address);
    warp::serve(routes).bind(address).await;

    Ok(())
}
