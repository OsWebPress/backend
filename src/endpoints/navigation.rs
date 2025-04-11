use actix_web::{web, HttpResponse, Responder, guard, http::header};
use crate::config;
use std::fs;
use std::path::Path;
use crate::jwt;

// register the endpoint.
pub fn navigation_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("navigation.vue")
        .route(web::get().to(get_navigation))
        // .route(web::post().to(post_navigation).guard(guard::fn_guard(auth_guard)))
        )
		.service(
			web::resource("navigation/data")
			.route(web::get().to(get_navigation_data))
			.route(web::post().to(post_navigation_data).guard(guard::fn_guard(auth_guard)))
		);

}

fn auth_guard(ctx: &guard::GuardContext) -> bool {
    if let Some(claims) = ctx.req_data().get::<jwt::Claims>() {
        println!("Claims: User Role: {}", claims.role);
    } else {
        println!("No claims found");
        return false;
    }
    return true;
}

async fn get_navigation(data: web::Data<config::PressConfig>) -> impl Responder {

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/default.vue", data.settings.root.clone());

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok().insert_header((header::CONTENT_TYPE, "application/javascript")).body(file_content),
        Err(_e) => HttpResponse::NotFound().body("Not found."),
    }
}

async fn get_navigation_data(data: web::Data<config::PressConfig>) -> impl Responder {

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/navigation.json", data.settings.root.clone());

    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => HttpResponse::Ok().insert_header((header::CONTENT_TYPE, "application/json")).body(file_content),
        Err(_e) => HttpResponse::NotFound().body("Not found."),
    }
}

// async fn post_navigation(req: HttpRequest, body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
//     let tail = req.match_info().get("tail").unwrap_or_default();

//     // Format the path to start with root and the file to be of type markdown.
//     let path = format!("{}/navigation/{}.md", data.settings.root.clone(), tail);
//     // now do something with the received body

//     // Get the directory part of the file path
//     let dir = Path::new(&path).parent().unwrap(); // Get the parent directory

//     // Create the directory if it doesn't exist
//     fs::create_dir_all(dir).expect("could not create dir");
//     let result = fs::write(path, body);

//     match result {
//         Ok(_) => HttpResponse::Ok(),
//         Err(_) => HttpResponse::InternalServerError(),
//     }
// }

async fn post_navigation_data(body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {

    // Format the path to start with root and the file to be of type markdown.
    let path = format!("{}/navigation/navigation.json", data.settings.root.clone());
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

