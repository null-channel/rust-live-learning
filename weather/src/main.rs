use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // Make request to get the
    let resp = reqwest::blocking::get("https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m&current=temperature_2m&timeformat=unixtime&wind_speed_unit=ms&temperature_unit=fahrenheit&precipitation_unit=inch")
        .expect("no worky")
        .json::<serde_json::Value>();
    println!("{resp:#?}");
}
