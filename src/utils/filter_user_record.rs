use crate::models::{FilteredUser, User};

pub fn filter_user_record(user: &User) -> FilteredUser {
	FilteredUser {
		id: user.id.to_string(),
		email: user.email.to_owned(),
		name: user.name.to_owned(),
		photo: user.photo.to_owned(),
		role: user.role.to_owned(),
		verified: user.verified,
		favourite: user.favourite.clone().unwrap_or(Vec::new()),
		createdAt: user.created_at.unwrap(),
		updatedAt: user.updated_at.unwrap(),
	}
}
