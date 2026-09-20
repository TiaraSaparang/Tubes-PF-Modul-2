use tubes_pf_modul_2::retry::{send_with_retry, SensorData};

#[tokio::main]
async fn main() {
    let data = SensorData {
        event_id: "event-retry-001".to_string(),
        device_id: "ESP32-001".to_string(),
        timestamp: "2026-09-20T13:00:00Z".to_string(),
        moisture: 72.5,
        temperature: 28.4,
        ph: 4.8,
        ec: 1.2,
    };

    let result = send_with_retry(
        "http://localhost:3000/api/sensor/data",
        "TubesPF-demo-key",
        &data,
        3,
    )
    .await;

    println!("Hasil pengiriman: {:?}", result);
}