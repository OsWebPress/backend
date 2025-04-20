use actix_web::{web, HttpRequest, HttpResponse, Responder, guard, http::header};
use crate::config;
use std::fs;
use std::path::Path;
use crate::endpoints::_auth;

// register the endpoint.
pub fn component_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("component/{tail:.*}")
        .route(web::get().to(get_component))
        .route(web::post().to(post_component).guard(guard::fn_guard(_auth::role_guard)))
        );
}

// _claims can be used for file permissions if we want to lock them in the future.
// will only lock editing them.
// we can make a test endpoint available which can be used to chekc if you can edit a file or add it into the data we send about all the files in the root for the editor.
async fn get_component(req: HttpRequest, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/component/{}", data.settings.root.clone(), tail);

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/javascript"))
            .body(file_content),
        Err(_e) => HttpResponse::NotFound().body("Not found."),
    }
}

async fn post_component(req: HttpRequest, body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/component/{}", data.settings.root.clone(), tail);
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

