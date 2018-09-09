extern crate actix_web;
use super::schema::categories;
use super::serde_json;
use super::serde::ser::Serialize;
use super::serde::de::Deserialize;
use self::actix_web::{HttpRequest, HttpResponse, Error, Responder, http};

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

/***************** Recipes ******************/


pub struct ApiResponse<T>(T);
impl <T> ApiResponse<T> {
	pub fn new(o: T) -> Self {
		ApiResponse(o)
	}
}
impl <T: Serialize> Responder for ApiResponse<T> {
	type Item = HttpResponse;
	type Error = Error;

	fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
		let body = serde_json::to_string(&self.0)?;

		Ok(HttpResponse::Ok()
			.content_type("application/json")
			.body(body)
		)
	}
}