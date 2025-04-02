use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::config;
use crate::database;
use crate::jwt;
use anyhow;
use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Argon2
};

#[derive(Serialize, Deserialize, Debug)]
pub struct EpUser {
    pub username: String,
    pub password: String,

	#[serde(default = "default_role")]
	pub role: String,
	#[serde(default = "default_id")]
	pub id: i32,
}

fn default_role() -> String {
	"User".to_string()
}

fn default_id() -> i32 {
	0
}

// register the login endpoint.
pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("login")
        .route(web::post().to(post_login))
        );
}

async fn verify_user(user: EpUser,  data: &web::Data<config::PressConfig>) -> anyhow::Result<database::User, anyhow::Error> {
	let db_pool = data.pool.clone().unwrap();
	let db_user = database::get_user(&db_pool, &user.username).await?;

	// verify the passwords for this user.
	let parsed_hash = PasswordHash::new(&db_user.password_hash).expect("error");
	let verified = Argon2::default().verify_password(user.password.as_bytes(), &parsed_hash);

	match verified {
		Ok(_res) => {
			return Ok(db_user)
		}
		Err(_e) => {
			return Err(anyhow::Error::msg("not ok"))
		}
	}
}

async fn post_login(body: web::Bytes, data: web::Data<config::PressConfig>) -> impl Responder {
	let user: EpUser = serde_json::from_slice(&body).unwrap();

	let verified = verify_user(user, &data).await;
	let jwt;

	// need to implement fmt::Display for role.
	match verified {
		Ok(user) => {
			jwt = jwt::create_jwt(user.id, user.role, &data.settings.jwt_secret);
		}
		Err(_e) => {
			return HttpResponse::Unauthorized().finish();
		}
	}

	match jwt {
		Ok(token) => HttpResponse::Ok().body(token),
		Err(_e) => HttpResponse::Unauthorized().finish(),
	}
}
