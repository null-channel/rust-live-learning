use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Current {
    pub apparent_temperature: f32,
    pub cloud_cover: i32,
    pub interval: i32,
    pub is_day: i32,
    pub precipitation: f32,
    pub pressure_msl: f32,
    pub rain: f32,
    pub relative_humidity_2m: f32,
    pub showers: f32,
    pub snowfall: f32,
    pub surface_pressure: f32,
    pub temperature_2m: f32,
    pub time: String,
    pub weather_code: i32,
    pub wind_direction_10m: i32,
    pub wind_gusts_10m: f32,
    pub wind_speed_10m: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub apparent_temperature: String,
    pub cloud_cover: String,
    pub interval: String,
    pub is_day: String,
    pub precipitation: String,
    pub pressure_msl: String,
    pub rain: String,
    pub relative_humidity_2m: String,
    pub showers: String,
    pub snowfall: String,
    pub surface_pressure: String,
    pub temperature_2m: String,
    pub time: String,
    pub weather_code: String,
    pub wind_direction_10m: String,
    pub wind_gusts_10m: String,
    pub wind_speed_10m: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherJson {
    pub current: Current,
    pub current_units: CurrentUnits,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i64,
}
