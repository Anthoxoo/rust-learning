use axum::{Router, routing::get};
use std::fs;

#[tokio::main]
async fn main() {
    let temperature: f32 = get_celcius_temperature();
    // build our application with a single route
    //let app = Router::new().route("/", get(|| async { "The temperature is currently at {}", temperature }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_celcius_temperature() -> f32 {
    let path: String = String::from("/sys/class/thermal/thermal_zone0");
    let raw_temperature = fs::read_to_string(path).expect("Error while reading the file");
    let final_temperature: f32 = raw_temperature
        .trim()
        .parse()
        .expect("error while converting to float");
    return final_temperature / 1000.0;
}
