use actix_web::{web, HttpRequest, HttpResponse, Responder, guard};
use crate::config;
use std::fs;
use crate::endpoints::_auth;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

// register the endpoint.
pub fn images_config(cfg: &mut web::ServiceConfig) {
    cfg
	.service(web::resource("images/upload")
	.app_data(web::PayloadConfig::new(1 << 25))
	.route(web::post().to(save_images).guard(guard::fn_guard(_auth::role_guard))))
	.service(
        web::resource("images/{tail:.*}")
        .route(web::get().to(get_images))
	);
}

async fn get_images(req: HttpRequest, data: web::Data<config::PressConfig>) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or_default();

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/images/{}", data.settings.root.clone(), tail);

    let content = fs::read(&path);
    match content {
        Ok(file_content) => HttpResponse::Ok().body(file_content),
        Err(_e) => {
			println!("path: {}", path);
			HttpResponse::NotFound().body("Not found.")
		}
    }
}

async fn save_images(mut payload: Multipart, data: web::Data<config::PressConfig>) -> impl Responder {
    while let Some(field) = payload.next().await {
        let mut field = field.unwrap();

        // Extract ContentDisposition from the field
        if let Some(content_disposition) = field.content_disposition() {
            // Call get_filename() on the ContentDisposition object itself
            if let Some(filename) = content_disposition.get_filename() {
                let filepath = format!("{}/images/{}", data.settings.root.clone(), filename);
                let mut f = File::create(filepath).unwrap();

                // Write the image data to file
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f.write_all(&data).unwrap();
                }
            } else {
			}
        }
    }

    HttpResponse::Ok().body("Image(s) uploaded successfully!")
}
