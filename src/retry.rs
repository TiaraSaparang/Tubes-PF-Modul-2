use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct SensorData {
    pub event_id: String,
    pub device_id: String,
    pub timestamp: String,
    pub moisture: f32,
    pub temperature: f32,
    pub ph: f32,
    pub ec: f32,
}

pub async fn send_with_retry(
    url: &str,
    api_key: &str,
    data: &SensorData,
    max_attempts: u32,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    for attempt in 1..=max_attempts {
        println!("Percobaan pengiriman ke-{}", attempt);

        let result = client
            .post(url)
            .header("X-API-Key", api_key)
            .json(data)
            .send()
            .await;

        match result {
            Ok(response) => {
                let status = response.status().as_u16();

                if !should_retry(Some(status)) {
                    if response.status().is_success() {
                        println!("Data berhasil dikirim.");
                        return Ok(());
                    }

                    return Err(format!(
                        "Pengiriman gagal dengan status HTTP {}",
                        status
                    ));
                }
            }

            Err(error) => {
                println!("Koneksi gagal: {}", error);

                if attempt == max_attempts {
                    return Err(
                        "Gagal mengirim data setelah beberapa percobaan".to_string()
                    );
                }
            }
        }

        if attempt < max_attempts {
            println!("Menunggu sebelum mencoba lagi...");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    Err("Pengiriman gagal".to_string())
}

pub fn should_retry(status_code: Option<u16>) -> bool {
    match status_code {
        Some(200) => false,
        Some(400) => false,
        Some(401) => false,
        Some(409) => false,
        Some(status) if status >= 500 => true,
        None => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy() {
        assert_eq!(should_retry(Some(200)), false);
        assert_eq!(should_retry(Some(400)), false);
        assert_eq!(should_retry(Some(401)), false);
        assert_eq!(should_retry(Some(409)), false);
        assert_eq!(should_retry(Some(500)), true);
        assert_eq!(should_retry(Some(503)), true);
        assert_eq!(should_retry(None), true);
    }
}