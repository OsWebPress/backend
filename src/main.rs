use actix_web::{App, HttpServer, web, HttpResponse};
use actix_web::middleware::from_fn;
mod endpoints;
mod config;
mod database;
mod jwt;

async fn handle_unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().body("Unauthorized!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get config info
    let mut config = config::parse_press_config();
    let pool = database::init_db(&mut config).await;
    config.pool = Some(pool);
    let state = web::Data::new(config);

    HttpServer::new(move || App::new().app_data(state.clone())
        .wrap(from_fn(jwt::middleware_decoder))
        .configure(endpoints::crud::crud_config)
        .configure(endpoints::login::login_config)
        .configure(endpoints::admin::admin_config)
        .default_service(web::route().to(handle_unauthorized))
        )
            .bind("127.0.0.1:8080")?
            .run()
            .await
}