mod api;
mod config;
mod controllers;
mod jwt_auth;
mod models;
mod utils;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use config::Config;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::controllers::auth::extract;

pub struct AppState {
	db: Pool<Postgres>,
	env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info");
	}
	dotenv().ok();
	env_logger::init();

	let config = Config::init();

	let pool = match PgPoolOptions::new()
		.max_connections(10)
		.connect(&config.database_url)
		.await
	{
		Ok(pool) => {
			println!("✅ Connection to the database is successful!");
			pool
		}
		Err(err) => {
			println!("🔥 Failed to connect to the database: {:?}", err);
			std::process::exit(1);
		}
	};

	println!("✅ Server started successfully on http://localhost:8080/api");

	HttpServer::new(move || {
		let auth = GrantsMiddleware::with_extractor(extract);
		App::new()
			.app_data(web::Data::new(AppState {
				db: pool.clone(),
				env: config.clone(),
			}))
			.configure(controllers::config)
			.wrap(Cors::permissive())
			.wrap(Logger::default())
			.wrap(auth)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
