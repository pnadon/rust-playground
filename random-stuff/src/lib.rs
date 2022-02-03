use std::error::Error;

pub type ResultErr<T> = Result<T, Box<dyn Error>>;
pub mod play_with_trait_objects;
pub mod population_api;
pub mod sunrise_sunset;
pub mod traits_and_boxes;
pub mod heapsort;
