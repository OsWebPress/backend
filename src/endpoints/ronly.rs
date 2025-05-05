use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::config;
use crate::utils::build_tree;
use std::path::PathBuf;

pub fn ronly_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
			web::resource("ronly/{tail:.*}")
			.route(web::get().to(get_file_structure))
		);
}

async fn get_file_structure(req: HttpRequest, data: web::Data<config::PressConfig>) -> impl Responder {
	let tail = req.match_info().get("tail").unwrap_or_default();
	let path = format!("{}/{}", data.settings.root.clone(), tail);
	let root_path = PathBuf::from(path);
	let file_tree = build_tree::build_tree(&root_path, &data.settings.root);

	let json = serde_json::to_string_pretty(&file_tree).unwrap();
	HttpResponse::Ok().body(json)
}
