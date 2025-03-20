use actix_web::{App, HttpServer, web};
mod endpoints;
mod config;
mod database;
mod jwt;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get config info
    let mut config = config::parse_press_config();
    let pool = database::init_db(&mut config).await;
    config.pool = Some(pool);
    let state = web::Data::new(config);

    HttpServer::new(move || App::new().app_data(state.clone())
        .configure(endpoints::crud::crud_config)
        .configure(endpoints::login::login_config))
            .bind("127.0.0.1:8080")?
            .run()
            .await
}