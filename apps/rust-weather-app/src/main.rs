use chrono::Utc;
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;
use rand::Rng;
use std::env;
use tokio;

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
    write_weather_data().await.unwrap();
}
