use axum::{
    extract::{Json, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
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

#[derive(Clone)]
struct AppState {
    api_key: String,
}

async fn sensor_handler(
   Json(payload): Json<SensorPayload>,
) -> impl IntoResponse {
    if let Err(message) = validate_payload(&payload) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message,
            }),
        );
    }

    println!("Data sensor diterima:");
    println!("{:?}", payload);

    (
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Sensor data received".to_string(),
        }),
    )
}

async fn authenticate(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let provided_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok());

    match provided_key {
        Some(key) if key == state.api_key => {
            next.run(request).await
        }
        _ => StatusCode::UNAUTHORIZED.into_response(),
    }
}

//batas temperatur dan ec masih sementara, nanti harus disesuaikan dengan rentang sensor di modul 1
fn validate_payload(payload: &SensorPayload) -> Result<(), String> {
    if payload.device_id.trim().is_empty() {
        return Err("device_id tidak boleh kosong".to_string());
    }

    if payload.moisture < 0.0 || payload.moisture > 100.0 {
        return Err("Moisture harus berada antara 0 dan 100".to_string());
    }

    if payload.temperature < -50.0 || payload.temperature > 100.0 {
        return Err("Temperature berada di luar batas yang diperbolehkan".to_string());
    }

    if payload.ph < 0.0 || payload.ph > 14.0 {
        return Err("pH harus berada antara 0 dan 14".to_string());
    }

    if payload.ec < 0.0 {
        return Err("EC tidak boleh bernilai negatif".to_string());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let state = AppState {
        api_key: "smartsoil-demo-key".to_string(),
    };

    let app = Router::new()
        .route("/api/sensor/data", post(sensor_handler))
        .route_layer(
            middleware::from_fn_with_state(
                state.clone(),
                authenticate,
            )
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server berjalan di http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}