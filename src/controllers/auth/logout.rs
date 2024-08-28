use crate::jwt_auth;
use actix_web::{
	cookie::{time::Duration as ActixWebDuration, Cookie},
	get, HttpResponse, Responder,
};
use serde_json::json;

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
	let cookie = Cookie::build("token", "")
		.path("/")
		.max_age(ActixWebDuration::new(-1, 0))
		.http_only(true)
		.finish();

	HttpResponse::Ok()
		.cookie(cookie)
		.json(json!({"status": "success"}))
}
