use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use serde;
use serde_json;

#[derive(serde::Deserialize)]
struct CityDefinition {
    code: String,
    city_name: String,
    connected_cities: Vec<(String, i32)>,
    refuel: bool
}
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    generate_cities();
}

fn generate_cities() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=data/cities.json");
    let cities_file = File::open("data/cities.json").expect("Cities File not Found!");
    let cities_buffer = BufReader::new(cities_file);
    let cities: Vec<CityDefinition> = serde_json::from_reader(cities_buffer).expect("Failed to Deserialize!");

    let mut cities_output = OpenOptions::new()
        .append(true)
        .open("src/cities.rs")
        .expect("Failed to Open File");
    std::fs::write(Path::new("src/cities.rs"),  "
use crate::types::city::City;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([\n").expect("Failed to Write to File");

    for city in cities {
        cities_output.write(format!("\t\t(\"{}\", City::new(\"{}\", &[", city.code, city.city_name).as_bytes()).expect("Failed to Write to File");
        for (code, distance) in city.connected_cities {
            cities_output.write(format!("(\"{}\", {}),", code, distance).as_bytes()).expect("Failed to Write to File");
        }
        cities_output.write(format!("], {})),\n", city.refuel).as_bytes()).expect("Failed to Write to File");
    }
    cities_output.write("
    ]);
}".as_bytes()).expect("Failed to Write to File");
}