// #[macro_use] extern crate serde_derive; 
// WHY ^^ 
extern crate foodb_api;
extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};
use foodb_api::models::{Category, ApiResponse};


fn index(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_categories()
	)
}

fn main() {
	server::new(|| App::new().resource("/", |r| r.f(index)))
		.bind("127.0.0.1:8000")
		.unwrap()
		.run();
}