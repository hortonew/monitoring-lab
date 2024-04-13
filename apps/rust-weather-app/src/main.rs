use chrono::Utc;
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;
use rand::Rng;
use reqwest;
use serde::Deserialize;
use std::env;
use tokio;

#[derive(Deserialize, Debug)]
struct FeatureResponse {
    enabled: bool,
}

#[derive(Default, Debug, WriteDataPoint)]
#[measurement = "weather"]
struct WeatherData {
    #[influxdb(tag)]
    zip_code: u64, // Unique city identifier as a tag
    #[influxdb(field)]
    temperature: f64, // Current temperature
    #[influxdb(u64)]
    humidity: u64,
    #[influxdb(timestamp)]
    time: i64, // Timestamp of the data in nanoseconds
}

async fn is_feature_enabled(feature_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Generic HTTP request to the Unleash API
    // SDK examples aren't working so use this as alternative
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://unleashweb:4242/api/client/features/{}", feature_name))
        .header("Authorization", "default:development.unleash-insecure-api-token")
        .send()
        .await?;

    let feature_response: FeatureResponse = response.json().await?;
    Ok(feature_response.enabled)
}

async fn write_weather_data() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("INFLUXDB_HOST").unwrap_or_else(|_| "http://influxdb2:8086".to_string());
    let org = env::var("INFLUXDB_ORG").unwrap_or_else(|_| "sre".to_string());
    let token = env::var("INFLUXDB_TOKEN").unwrap_or_else(|_| "AnInsecureTokenYouShouldProbablyChangeThis".to_string());
    let bucket = "weather";

    let client = Client::new(host, org, token);
    let random_weather = vec![WeatherData {
        zip_code: 10001,
        temperature: rand::thread_rng().gen_range(0.0..100.0),
        humidity: rand::thread_rng().gen_range(10..60),
        time: Utc::now().timestamp_nanos_opt().unwrap(),
    }];

    println!("{:?}", &random_weather);
    client.write(bucket, stream::iter(random_weather)).await?;

    Ok(())
}
#[tokio::main]
async fn main() {
    let feature_name = "weather_data";

    match is_feature_enabled(feature_name).await {
        Ok(enabled) => {
            if enabled {
                println!("Feature '{}' is enabled.", feature_name);
                write_weather_data().await.unwrap();
            } else {
                println!("Feature '{}' is disabled.", feature_name);
            }
        }
        Err(e) => eprintln!("Error checking feature flag: {}", e),
    }
}
