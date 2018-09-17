// #[macro_use] extern crate serde_derive; 
// WHY ^^ 
extern crate foodb_api;
extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder, http};
use actix_web::{Json};
use actix_web::middleware::cors::Cors;
use foodb_api::models::{NewCategory, NewVerb, NewTool, NewContainer, ApiResponse};

/*********************** categories *******************/

fn categories_get(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_categories()
	)
}

fn categories_post(payload: Json<NewCategory>) -> impl Responder {
	let new_category = NewCategory{
		name: payload.name.clone()
	};

	foodb_api::create_category(&new_category);

	ApiResponse::new(new_category)
}

/*********************** verbs *******************/

fn verbs_get(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_verbs()
	)
}

fn verbs_post(payload: Json<NewVerb>) -> impl Responder {
	let new_verb = NewVerb {
		verb: payload.verb.clone()
	};

	foodb_api::create_verb(&new_verb);

	ApiResponse::new(new_verb)
}

/*********************** tools *******************/
fn tools_get(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_tools()
	)
}

fn tools_post(payload: Json<NewTool>) -> impl Responder {
	let new_tool = NewTool {
		name: payload.name.clone()
	};

	foodb_api::create_tool(&new_tool);

	ApiResponse::new(new_tool)
}
/*********************** containers *******************/
fn containers_get(_req: &HttpRequest) -> impl Responder {
	ApiResponse::new(
		foodb_api::get_containers()
	)
}

fn containers_post(payload: Json<NewContainer>) -> impl Responder {
	let new_container = NewContainer {
		name: payload.name.clone()
	};

	foodb_api::create_container(&new_container);

	ApiResponse::new(new_container)
}

fn main() {
    server::new(|| App::new().configure(|app| {
        Cors::for_app(app)
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .resource("/categories", |r| {
            	r.get().f(categories_get);
            	r.post().with(categories_post)
            })
            .resource("/verbs", |r| {
            	r.get().f(verbs_get);
            	r.post().with(verbs_post)
            })
            .resource("/tools", |r| {
            	r.get().f(tools_get);
            	r.post().with(tools_post)
            })
            .resource("/containers", |r| {
            	r.get().f(containers_get);
            	r.post().with(containers_post)
            })
            .register()
    }))
    .bind("127.0.0.1:8001")
    .unwrap()
    .run();
}