use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::database::Role;
use crate::config;
use anyhow;
use actix_web::{
	HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
	user_id: i32,
	pub role: Role,
	exp: usize,
}

pub fn create_jwt(user_id: i32, role: &Role, secret: &String) ->anyhow::Result<String, anyhow::Error> {
    let expiration = Utc::now().timestamp() as usize + 3600; // 1 hour from now
    let my_claims = Claims { user_id: user_id, role: role.clone(), exp: expiration };
    let key = EncodingKey::from_secret(secret.as_ref());
	// get secret fromt eh db

    // encode(&Header::default(), &my_claims, &key).unwrap()
	let token = encode(&Header { alg: Algorithm::HS256, ..Default::default() }, &my_claims, &key);

	match token {
		Ok(webtoken) => {
			return anyhow::Ok(webtoken);
		}
		Err(_e) => {
			return Err(anyhow::Error::msg("webtoken error"));
		}
	}
}

pub async fn middleware_decoder(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
	let data = req.app_data::<actix_web::web::Data<config::PressConfig>>();
	let conf;
	match data {
		Some(configdata) => {
			conf = (*configdata).clone();
		}
		None => {
			// bit ugly to early return here
			return next.call(req).await
		}
	}
	// let key = DecodingKey::from_secret(config_data.get_ref().as_ref());
	let key = DecodingKey::from_secret(conf.settings.jwt_secret.as_ref());
	let validation = Validation::new(Algorithm::HS256);

	if let Some(cookie) = req.cookie("jwt_token") {
		let token_str = cookie.value();

		if let Ok(token) = decode::<Claims>(token_str, &key, &validation) {
			// Optionally check expiration here manually if needed,
			// or ensure `validation.validate_exp = true`

			req.extensions_mut().insert(token.claims);
		}
	}

	// Call the actual handler (or next middleware).
    next.call(req).await
}
