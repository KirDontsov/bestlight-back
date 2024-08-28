use crate::models::{Counter, SaveCounter};
use sqlx::{Pool, Postgres};

pub async fn get_counter(db: &Pool<Postgres>, id: &String) -> i64 {
	let counter_query_result = Counter::get_counter(db, id).await.unwrap();

	counter_query_result
		.value
		.clone()
		.unwrap()
		.parse::<i64>()
		.unwrap()
}

pub async fn update_counter(db: &Pool<Postgres>, id: &String, value: &String) -> Counter {
	let counter_query_result = Counter::update_counter(
		db,
		SaveCounter {
			counter_id: uuid::Uuid::parse_str(&id).unwrap(),
			value: value.clone(),
		},
	)
	.await
	.unwrap();

	counter_query_result
}
