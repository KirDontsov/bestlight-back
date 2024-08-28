use crate::{
	models::{LoginUserSchema, TokenClaims, User},
	AppState,
};
use actix_web::cookie::SameSite;
use actix_web::{
	cookie::{time::Duration as ActixWebDuration, Cookie},
	post, web, HttpResponse, Responder,
};
use argon2::{
	password_hash::{PasswordHash, PasswordVerifier},
	Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

#[post("/auth/login")]
async fn login_handler(
	body: web::Json<LoginUserSchema>,
	data: web::Data<AppState>,
) -> impl Responder {
	let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
		.fetch_optional(&data.db)
		.await
		.unwrap();

	let is_valid = query_result.to_owned().map_or(false, |user| {
		let parsed_hash = PasswordHash::new(&user.password).unwrap();
		Argon2::default()
			.verify_password(body.password.as_bytes(), &parsed_hash)
			.map_or(false, |_| true)
	});

	if !is_valid {
		return HttpResponse::BadRequest().json(
			json!({"status": "fail", "message": "Неправильный адрес электронной почты или пароль"}),
		);
	}

	let user = query_result.unwrap();

	let now = Utc::now();
	let iat = now.timestamp() as usize;
	let exp = (now + Duration::minutes(60)).timestamp() as usize;
	let claims: TokenClaims = TokenClaims {
		sub: user.id.to_string(),
		role: user.role.to_string(),
		exp,
		iat,
	};

	let token = encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
	)
	.unwrap();

	let cookie = Cookie::build("token", token.to_owned())
		.same_site(SameSite::None)
		.path("/")
		.max_age(ActixWebDuration::new(60 * 60, 0))
		.http_only(true)
		.secure(true)
		.finish();

	HttpResponse::Ok()
		.cookie(cookie)
		.json(json!({"status": "success", "token": token}))
}
