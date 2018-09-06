#[macro_use] extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::Category;

pub fn establish_connection() -> PgConnection {

	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn get_categories() -> Vec<Category> {
	use schema::categories::dsl::*;
	let connection = establish_connection();

	let results = categories
		.load::<Category>(&connection)
		.expect("Error loading categories");
	results
}