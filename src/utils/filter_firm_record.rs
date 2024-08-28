use crate::models::{
	ExtFilteredFirmWithOaiDescription, ExtFirmWithOaiDescription, FilteredFirm, FilteredFirmForMap,
	Firm, FirmForMap,
};

pub fn filter_firm_record(firm: &Firm) -> FilteredFirm {
	FilteredFirm {
		firm_id: firm.firm_id.to_string(),
		two_gis_firm_id: firm.two_gis_firm_id.to_owned(),
		city_id: firm.city_id.to_string(),
		category_id: firm.category_id.to_string(),
		name: firm.name.to_owned(),
		description: firm.description.to_owned(),
		address: firm.address.to_owned(),
		site: firm.site.to_owned(),
		rating: firm.rating.to_owned(),
		reviews_count: firm.reviews_count.to_owned(),
		default_phone: firm.default_phone.to_owned(),
		url: firm.url.to_owned(),
		coords: firm.coords.to_owned(),
	}
}

// depricated
pub fn filter_ext_firm_record(
	firm: &ExtFirmWithOaiDescription,
) -> ExtFilteredFirmWithOaiDescription {
	ExtFilteredFirmWithOaiDescription {
		firm_id: firm.firm_id.to_string(),
		category_id: firm.category_id.to_string(),
		name: firm.name.to_owned(),
		address: firm.address.to_owned(),
		site: firm.site.to_owned(),
		default_phone: firm.default_phone.to_owned(),
		oai_description_value: firm.oai_description_value.to_owned(),
		description: firm.description.to_owned(),
	}
}

pub fn filter_firm_for_map_record(firm: &FirmForMap) -> FilteredFirmForMap {
	FilteredFirmForMap {
		name: firm.name.to_owned(),
		address: firm.address.to_owned(),
		url: firm.url.to_owned(),
		coords: firm.coords.to_owned(),
	}
}
