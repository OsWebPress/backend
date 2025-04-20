use actix_web::{web, HttpResponse, Responder, guard};
use crate::config;
use crate::database::Role;
use crate::database;
use crate::endpoints::{login, _auth};

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("admin")
            .guard(guard::fn_guard(_auth::admin_guard))
            .route("/users", web::get().to(get_users))
            .route("user", web::post().to(post_user))
			.route("/user", web::delete().to(delete_user))
        );
}

// get a list of all users.
async fn get_users(data: web::Data<config::PressConfig>) -> impl Responder {
	let db_pool = data.pool.clone().unwrap();

	// get the user password / username and role fromt the body
	let res = database::get_all_users(&db_pool).await;
	match res {
		Ok(users) => {
			return HttpResponse::Ok().json(users)
		}
		Err(_e) => {
			return HttpResponse::InternalServerError().finish()
		}
	}
}

// create or update user.
async fn post_user(body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
	// verify the body as json with username password and a role.
	let user: login::EpUser = serde_json::from_slice(&body).unwrap();

	let db_pool = data.pool.clone().unwrap();

	// get the user password / username and role fromt the body
	let res = database::add_user(&db_pool, &user.username, &user.password, Role::from_str(&user.role)).await;
	match res {
		Ok(_a) => {
			return HttpResponse::Ok().finish()
		}
		Err(_e) => {
			return HttpResponse::InternalServerError().finish()
		}
	}
}

async fn delete_user(body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
	let user: login::EpUser = serde_json::from_slice(&body).unwrap();

	let db_pool = data.pool.clone().unwrap();

	// get the user password / username and role fromt the body
	let res = database::delete_user(&db_pool, &user.username).await;
	match res {
		Ok(_a) => {
			return HttpResponse::Ok().finish()
		}
		Err(_e) => {
			return HttpResponse::InternalServerError().finish()
		}
	}
}
