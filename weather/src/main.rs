pub mod weather {
    tonic::include_proto!("weather");
}
pub mod weather_api;

use weather::weather_server::{Weather, WeatherServer};

use crate::weather_api::WeatherJson;

pub struct WeatherService;

#[tonic::async_trait]
impl Weather for WeatherService {
    async fn get_weather(
        &self,
        request: tonic::Request<weather::GetWeatherRequest>,
    ) -> Result<tonic::Response<weather::GetWeatherResponse>, tonic::Status> {
        // make request to external weather API
        let request_url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,is_day,apparent_temperature,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&timezone=America%2FNew_York&forecast_days=1",
            request.get_ref().latitude,
            request.get_ref().longitude,
        );

        let weather_json = reqwest::blocking::get(request_url)
            .expect("API Call Failed")
            .json::<WeatherJson>()
            .expect("Failed to parse JSON");

        let response = weather::GetWeatherResponse {
            temperature: weather_json.current.apparent_temperature,
            humidity: weather_json.current.relative_humidity_2m,
            pressure: weather_json.current.pressure_msl,
            wind_speed: weather_json.current.wind_speed_10m,
            wind_direction: weather_json.current.wind_direction_10m,
            cloud_cover: weather_json.current.cloud_cover,
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the gRPC server
    let addr = "[::1]:50001".parse().unwrap();
    let weather_service = WeatherService {};
    let svc = WeatherServer::new(weather_service);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;
    Ok(())
}
