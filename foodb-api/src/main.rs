// #[macro_use] extern crate serde_derive; 
// WHY ^^ 
extern crate foodb_api;
extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};
use actix_web::{Json};
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use foodb_api::models::{Category, NewCategory, ApiResponse};



fn categories_get(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_categories()
	)
}

fn categories_post(payload: Json<NewCategory>) -> impl Responder {
	println!("Got new category: {}", payload.name);

	ApiResponse::new(
		NewCategory{
			name: payload.name.clone()
		}
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
            .resource("/categories", |r| {
            	r.get().f(categories_get);
            	r.post().with(categories_post)
            })
            .register()
    }))
    .bind("127.0.0.1:8001")
    .unwrap()
    .run();
}