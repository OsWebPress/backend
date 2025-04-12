use actix_web::{web, HttpResponse, Responder};
use crate::config;
use crate::utils::build_tree;
use std::path::PathBuf;

pub fn ronly_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("ronly/files")
        .route(web::get().to(get_file_structure))
        );
}

async fn get_file_structure(data: web::Data<config::PressConfig>) -> impl Responder {
	let root_path = PathBuf::from(data.settings.root.clone());
	let file_tree = build_tree::build_tree(&root_path);

	let json = serde_json::to_string_pretty(&file_tree).unwrap();
	HttpResponse::Ok().body(json)
}
