use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SensorPayload {
    device_id: String,
    timestamp: String,
    moisture: f32,
    temperature: f32,
    ph: f32,
    ec: f32,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

async fn sensor_handler(
    Json(payload): Json<SensorPayload>,
) -> impl IntoResponse {

    println!("Data sensor diterima:");
    println!("{:#?}", payload);

    (
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Sensor data received".to_string(),
        }),
    )
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/api/sensor/data", post(sensor_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server berjalan di http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}