use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::config;
use std::fs;
use std::path::Path;

// register the endpoint.
pub fn crud_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("carbon/{tail:.*}")
        .route(web::get().to(get_carbon))
        .route(web::post().to(post_carbon))
        );
}

// async fn test_handler(data: web::Data<config::PressConfig>) -> impl Responder {
//     HttpResponse::Ok().body(data.settings.root.clone())
// }

async fn get_carbon(req: HttpRequest, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/{}.md", data.settings.root.clone(), tail);

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok().body(file_content),
        Err(E) => HttpResponse::NotFound().body("Not found."),
    }
}

async fn post_carbon(req: HttpRequest, body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/{}.md", data.settings.root.clone(), tail);
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

