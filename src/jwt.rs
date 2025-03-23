use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::database::Role;
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

pub fn create_jwt(user_id: i32, role: Role) ->anyhow::Result<String, anyhow::Error> {
    let expiration = Utc::now().timestamp() as usize + 3600; // 1 hour from now
    let my_claims = Claims { user_id: user_id, role: role, exp: expiration };
    let key = EncodingKey::from_secret("secret_key".as_ref());
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
	// here we need to decode the jwt if it is there and if not do nothing.
    let key = DecodingKey::from_secret("secret_key".as_ref());
	let validation = Validation::new(Algorithm::HS256);

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
			if let Ok(token) = decode::<Claims>(auth_str, &key, &validation)
				{
					// check if the JWT has expired yet.
					// needs to not handle this when someone is trying to login while having an outdated token.

					req.extensions_mut().insert(token.claims);
				}
        }
	}

	// Call the actual handler (or next middleware).
    next.call(req).await
}
