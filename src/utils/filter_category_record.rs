use crate::models::{Category, FilteredCategory};

pub fn filter_category_record(category: &Category) -> FilteredCategory {
	FilteredCategory {
		category_id: category.category_id.to_string(),
		name: category.name.to_owned(),
		abbreviation: category.abbreviation.to_owned(),
		is_active: category.is_active.to_owned(),
	}
}
