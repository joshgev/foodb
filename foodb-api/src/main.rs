// #[macro_use] extern crate serde_derive; 
// WHY ^^ 
extern crate foodb_api;
extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};
use foodb_api::models::{Category, ApiResponse};
use actix_web::middleware::cors::Cors;


fn index(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_categories()
	)
}

fn main() {
    // let app = ;
    server::new(|| App::new().configure(|app| {
        Cors::for_app(app) // <- Construct CORS middleware builder
            // .allowed_origin("*")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .resource("/categories", |r| r.f(index))
            .register()
    }))
    .bind("127.0.0.1:8001")
    .unwrap()
    .run();
}