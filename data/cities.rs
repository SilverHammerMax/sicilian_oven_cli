use crate::types::city::City;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([