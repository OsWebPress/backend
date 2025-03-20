use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{ FromRow, PgPool, Result};
use sqlx;
use crate::config;
use time;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use chrono::{DateTime, Utc}; // Ensure chrono is in Cargo.toml

#[derive(Serialize, Deserialize)]
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    Mod,
    User,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

pub async fn init_db(config: &mut config::PressConfig) -> PgPool {
    let pool = init_pool(config).await;
	setup_schema(&pool).await;

	// add the default user as moderator?
	add_default_user(&pool, config).await;

	let user_one = get_user(&pool, "user").await;
	match user_one {
        Ok(user) => {
            println!("username: {}, password (hashed?) {}", user.username, user.password_hash);
		}
		Err(e) => {
            println!("ERROR could not find the user! error: {:?}", e)
		}
	}

	pool
}

async fn init_pool(config: &config::PressConfig) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5) // Set max connections
        .connect(&config.settings.database_url)
        .await
        .expect("Failed to connect to the database")
}

async fn setup_schema(pool: &PgPool) {
	let role_sql_create = r#"
		DO $$
		BEGIN
			IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
				CREATE TYPE role AS ENUM ('admin', 'mod', 'user');
			END IF;
		END $$;
	"#;

    let user_create = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
			role role DEFAULT 'user' NOT NULL,
            created_at TIMESTAMPTZ DEFAULT now()
        )
    "#;

    sqlx::query(role_sql_create)
        .execute(pool)
        .await
        .expect("Failed to create auth level enum");

    sqlx::query(user_create)
        .execute(pool)
        .await
        .expect("Failed to create users table");
}

pub async fn add_user(pool: &PgPool, username: &str, password: &str, role: Role) -> Result<(), sqlx::Error> {
	let now: DateTime<Utc> = Utc::now();
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();
	let password_hash = argon2.hash_password(password.as_bytes(), &salt).expect("Failed to hash password.").to_string();


    // Insert user, ignore if username already exists
    sqlx::query(
        "INSERT INTO users (username, password_hash, role, created_at)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (username)
		 DO UPDATE SET
			password_hash = EXCLUDED.password_hash,
			role = EXCLUDED.role"
    )
    .bind(username)
    .bind(password_hash)
    .bind(role)
	.bind(now)
    .execute(pool)
    .await.expect("failed to create user");

    Ok(())
}

pub async fn get_user(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
	let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1;")
		.bind(username)
		.fetch_one(pool)
		.await;

	result
}

async fn add_default_user(pool: &PgPool, config: &mut config::PressConfig) {
	add_user(pool, &config.settings.username, &config.settings.password, Role::Admin)
	.await
    .expect("Failed to insert default user");
	// read the password and username from config
	// use the pool to add it to the database.
}