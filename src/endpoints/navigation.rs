use actix_web::{web, HttpRequest, HttpResponse, Responder, guard, http::header};
use crate::config;
use std::fs;
use std::path::Path;
use crate::endpoints::_auth;

// register the endpoint.
pub fn navigation_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("navigation/active.vue")
        .route(web::get().to(get_active_navigation))
        // .route(web::post().to(post_navigation).guard(guard::fn_guard(auth_guard)))
        )
		.service(
			web::resource("navigation/{tail:.*}")
			.route(web::get().to(get_navigation))
			.route(web::post().to(post_navigation).guard(guard::fn_guard(_auth::role_guard)))
		);
}

/* Not REST but can be used to get the active navigation component with some DB-fu. */
async fn get_active_navigation(data: web::Data<config::PressConfig>) -> impl Responder {

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/default.vue", data.settings.root.clone());

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok().insert_header((header::CONTENT_TYPE, "application/javascript")).body(file_content),
        Err(_e) => HttpResponse::NotFound().body("Not found."),
    }
}

async fn get_navigation(req: HttpRequest, data: web::Data<config::PressConfig>) -> impl Responder {
	let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/{}", data.settings.root.clone(), tail);

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok().insert_header((header::CONTENT_TYPE, "application/json")).body(file_content),
        Err(_e) => HttpResponse::NotFound().body("Not found."),
    }
}

async fn post_navigation(req: HttpRequest, body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/{}", data.settings.root.clone(), tail);
    // now do something with the received body

    // Get the directory part of the file path
    let dir = Path::new(&path).parent().unwrap(); // Get the parent directory

    // Create the directory if it doesn't exist
    fs::create_dir_all(dir).expect("could not create dir");
    let result = fs::write(path, body);

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
