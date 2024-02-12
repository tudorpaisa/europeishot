use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "weather.locations")]
pub struct Location {
    pub city_id: i32,
    pub country_name: String,
    pub city_name: String,
    pub lat: f32,
    pub long: f32,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "weather.data")]
pub struct Data {
    pub id: i32,
    pub temperature_c: f32,
    pub city_id: i32,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "LatestWeatherData")]
pub struct LatestWeatherData {
    pub city_name: String,
    pub country_name: String,
    pub temperature_c: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Sys {
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

#[derive(Deserialize, Serialize)]
pub struct WeatherResult {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub name: String,
    pub cod: i32,
}
