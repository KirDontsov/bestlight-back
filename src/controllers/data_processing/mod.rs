pub mod description_processing;
pub mod images_processing;
pub mod reviews_count_processing;
pub mod reviews_processing;
pub mod sitemap_processing;
pub mod urls_processing;

pub use self::description_processing::description_processing_handler;
pub use self::images_processing::images_processing_handler;
pub use self::reviews_count_processing::reviews_count_processing_handler;
pub use self::reviews_processing::reviews_processing_handler;
pub use self::sitemap_processing::sitemap_processing_handler;
pub use self::urls_processing::urls_processing_handler;
