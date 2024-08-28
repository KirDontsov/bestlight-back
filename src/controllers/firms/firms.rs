use crate::{
	models::{Category, City, Count, FilterExtOptions, FilteredFirm, FilteredFirmForMap, Firm},
	utils::filter_firm_record::{filter_firm_for_map_record, filter_firm_record},
	AppState,
};
use actix_web::{
	get,
	web::{self, Path},
	HttpResponse, Responder,
};
use serde_json::json;
use uuid::Uuid;

#[get("/firms_by_abbr")]
async fn get_firms_by_abbr_handler(
	opts: web::Query<FilterExtOptions>,
	data: web::Data<AppState>,
) -> impl Responder {
	let limit = opts.limit.unwrap_or(10);
	let offset = (opts.page.unwrap_or(1) - 1) * limit;
	let city_id = opts.city_id.clone().expect("expect opts_city_id");
	let category_id = opts.category_id.clone().expect("expect opts_category_id");
	let table = String::from("firms");

	let city_query_result = City::get_city_by_abbr(&data.db, &city_id).await;
	let city_message = "Что-то пошло не так во время чтения city";
	if city_query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": &city_message}));
	}
	let city = city_query_result.expect(&city_message);

	let category_query_result = Category::get_category_by_abbr(&data.db, &category_id).await;
	let category_message = "Что-то пошло не так во время чтения category";
	if category_query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": &category_message}));
	}
	let category = category_query_result.expect(&category_message);

	let firms = Firm::get_firms_by_city_catagory(
		&data.db,
		city.city_id,
		category.category_id,
		limit as i32,
		offset as i32,
	)
	.await
	.expect("Что-то пошло не так во время чтения firms");

	let firms_count = Count::count_firms_by_city_category(
		&data.db,
		table,
		city.city_id.clone(),
		category.category_id.clone(),
	)
	.await
	.unwrap_or(0);

	let json_response = json!({
		"status":  "success",
		"data": json!({
			"firms": &firms.into_iter().map(|firm| filter_firm_record(&firm)).collect::<Vec<FilteredFirm>>(),
			"firms_count": &firms_count
		})
	});

	HttpResponse::Ok().json(json_response)
}

#[get("/firm/{id}")]
async fn get_firm_handler(path: Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
	let firm_id = &path.into_inner();

	let firm = sqlx::query_as::<_, Firm>("SELECT * FROM firms WHERE firm_id = $1")
		.bind(firm_id)
		.fetch_one(&data.db)
		.await
		.unwrap();

	let json_response = json!({
		"status":  "success",
		"data": json!({
			"firm": filter_firm_record(&firm)
		})
	});

	HttpResponse::Ok().json(json_response)
}

#[get("/firm_by_url/{id}")]
async fn get_firm_by_url_handler(path: Path<String>, data: web::Data<AppState>) -> impl Responder {
	let firm_url = &path.into_inner();

	let firm_query_result = Firm::get_firm_by_url(&data.db, &firm_url).await;
	let firm_message = "Что-то пошло не так во время чтения get_firm_by_url";
	if firm_query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": &firm_message}));
	}

	let firm = firm_query_result.expect(&firm_message);

	let json_response = json!({
		"status":  "success",
		"data": json!({
			"firm": filter_firm_record(&firm)
		})
	});

	HttpResponse::Ok().json(json_response)
}

#[get("/firms_by_abbr_for_map")]
async fn get_firms_by_abbr_for_map_handler(
	opts: web::Query<FilterExtOptions>,
	data: web::Data<AppState>,
) -> impl Responder {
	let city_id = opts.city_id.clone().expect("expect opts_city_id");
	let category_id = opts.category_id.clone().expect("expect opts_category_id");
	let table = String::from("firms");

	let city_query_result = City::get_city_by_abbr(&data.db, &city_id).await;
	let city_message = "Что-то пошло не так во время чтения city";
	if city_query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": &city_message}));
	}

	let city = city_query_result.expect(&city_message);

	let category_query_result = Category::get_category_by_abbr(&data.db, &category_id).await;
	let category_message = "Что-то пошло не так во время чтения category";
	if category_query_result.is_err() {
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": &category_message}));
	}

	let category = category_query_result.expect(&category_message);

	let firms = Firm::get_firms_by_city_catagory_for_map(
		&data.db,
		city.city_id.clone(),
		category.category_id.clone(),
	)
	.await
	.expect("Что-то пошло не так во время чтения firms");

	let firms_count = Count::count_firms_by_city_category(
		&data.db,
		table,
		city.city_id.clone(),
		category.category_id.clone(),
	)
	.await
	.unwrap_or(0);

	let json_response = json!({
		"status":  "success",
		"data": json!({
			"firms": &firms.into_iter().map(|firm| filter_firm_for_map_record(&firm)).collect::<Vec<FilteredFirmForMap>>(),
			"firms_count": &firms_count
		})
	});

	HttpResponse::Ok().json(json_response)
}

#[get("/firms_search")]
async fn get_firms_search_handler(
	opts: web::Query<FilterExtOptions>,
	data: web::Data<AppState>,
) -> impl Responder {
	let input = opts.input.clone().unwrap_or("".to_string());

	let query_result = sqlx::query_as::<_, Firm>(
		"SELECT * FROM firms
		WHERE ts @@ phraseto_tsquery('english', $1)
		ORDER BY ts_rank(ts, phraseto_tsquery('english', $1)) DESC;",
	)
	.bind(input.clone())
	.fetch_all(&data.db)
	.await;

	if query_result.is_err() {
		let message = "Что-то пошло не так во время чтения firms";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}

	let firms = query_result.expect("Что-то пошло не так во время чтения firms");

	let json_response = json!({
		"status":  "success",
		"data": json!({
			"firms": &firms.into_iter().map(|firm| filter_firm_record(&firm)).collect::<Vec<FilteredFirm>>(),
		})
	});

	HttpResponse::Ok().json(json_response)
}
