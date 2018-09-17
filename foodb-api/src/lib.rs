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

use models::{Category, NewCategory, Verb, NewVerb, Tool, NewTool, Container, NewContainer};

pub fn establish_connection() -> PgConnection {

	dotenv().ok();

	// let database_url = env::var("DATABASE_URL")
	// 	.expect("DATABASE_URL must be set");

	let database_url = String::from("postgresql://foodb:foodb@localhost");
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

/********************** categories ****************/
pub fn get_categories() -> Vec<Category> {
	use schema::categories::dsl::*;
	let connection = establish_connection();

	categories
		.load::<Category>(&connection)
		.expect("Error loading categories")
}

pub fn create_category(new_category: &NewCategory) -> Category {
	use schema::categories;
	let conn = establish_connection();

	diesel::insert_into(categories::table)
		.values(new_category)
		.get_result(&conn)
		.expect("Error creating new category")
}

/********************** verbs ****************/

pub fn get_verbs () -> Vec<Verb> {
	use schema::verbs::dsl::*;
	let connection = establish_connection();

	verbs
		.load::<Verb>(&connection)
		.expect("Error loading verbs")
}

pub fn create_verb(new_verb: &NewVerb) -> Verb {
	use schema::verbs;
	let conn = establish_connection();

	diesel::insert_into(verbs::table)
		.values(new_verb)
		.get_result(&conn)
		.expect("Error creating new verb")
}

/********************** tools ****************/
pub fn get_tools () -> Vec<Tool> {
	use schema::tools::dsl::*;
	let connection = establish_connection();

	tools
		.load::<Tool>(&connection)
		.expect("Error loading tools")
}

pub fn create_tool(new_tool: &NewTool) -> Tool {
	use schema::tools;
	let conn = establish_connection();

	diesel::insert_into(tools::table)
		.values(new_tool)
		.get_result(&conn)
		.expect("Error creating new tool")
}
/********************** containers ****************/
pub fn get_containers () -> Vec<Container> {
	use schema::containers::dsl::*;
	let connection = establish_connection();

	containers
		.load::<Container>(&connection)
		.expect("Error loading containers")
}

pub fn create_container(new_container: &NewContainer) -> Container {
	use schema::containers;
	let conn = establish_connection();

	diesel::insert_into(containers::table)
		.values(new_container)
		.get_result(&conn)
		.expect("Error creating new container")
}