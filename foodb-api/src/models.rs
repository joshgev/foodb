extern crate actix_web;
use super::schema::{categories, verbs, tools, containers};
use super::serde_json;
use super::serde::ser::Serialize;
use self::actix_web::{HttpRequest, HttpResponse, Error, Responder};

/***************** Categories ******************/
#[derive(Queryable, Serialize)]
pub struct Category {
	pub category_id: i32,
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="categories"]
pub struct NewCategory {
	pub name: String
}

/***************** Verbs ******************/
#[derive(Queryable, Serialize)]
pub struct Verb {
	pub verb_id: i32,
	pub verb: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="verbs"]
pub struct NewVerb {
	pub verb: String
}

/***************** Tools ******************/
#[derive(Queryable, Serialize)]
pub struct Tool {
	pub tool_id: i32,
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="tools"]
pub struct NewTool {
	pub name: String
}

/***************** Containers ******************/
#[derive(Queryable, Serialize)]
pub struct Container {
	pub container_id: i32,
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="containers"]
pub struct NewContainer {
	pub name: String
}


pub struct ApiResponse<T>(T);
impl <T> ApiResponse<T> {
	pub fn new(o: T) -> Self {
		ApiResponse(o)
	}
}
impl <T: Serialize> Responder for ApiResponse<T> {
	type Item = HttpResponse;
	type Error = Error;

	fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
		let body = serde_json::to_string(&self.0)?;

		Ok(HttpResponse::Ok()
			.content_type("application/json")
			.body(body)
		)
	}
}