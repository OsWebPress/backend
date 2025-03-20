use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::database::Role;
use anyhow;

#[derive(Serialize, Deserialize)]
pub struct Claims {
	user_id: i32,
	role: Role,
	expires: usize,
}

pub fn create_jwt(user_id: i32, role: Role) ->anyhow::Result<String, anyhow::Error> {
    let expiration = Utc::now().timestamp() as usize + 3600; // 1 hour from now
    let my_claims = Claims { user_id: user_id, role: role, expires: expiration };
    let key = EncodingKey::from_secret("secret".as_ref());
	// get secret fromt eh db

    // encode(&Header::default(), &my_claims, &key).unwrap()
	let token = encode(&Header::default(), &my_claims, &key);

	match token {
		Ok(webtoken) => {
			return anyhow::Ok(webtoken);
		}
		Err(_e) => {
			return Err(anyhow::Error::msg("webtoken error"));
		}
	}
}