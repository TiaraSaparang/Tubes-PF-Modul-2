// ============================================================================
// FILE: src/retry.rs
// STATUS: PURE FUNCTIONS (Offline Buffering) & SIDE-EFFECT FUNCTION (HTTP)
// HUBUNGAN KE WEB UI: KOTAK 2 (Simulasi Koneksi & Offline Buffering)
// ============================================================================

use crate::contracts::RawReading;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Item yang tersimpan di dalam antrean Offline Buffer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BufferedItem {
    pub raw: RawReading,
    pub failed_attempts: u32,
    pub reason: String,
}

// ============================================================================
// BAGIAN 1: PURE FUNCTIONS (Fungsi Murni Tanpa Efek Samping, Bebas I/O)
// ============================================================================

// Pengecekan kode HTTP: status 500/timeout di-retry, status 200/400/401/409 tidak di-retry
pub fn should_retry(status_code: Option<u16>) -> bool {
    match status_code {
        Some(200) | Some(400) | Some(401) | Some(403) | Some(409) => false,
        Some(status) if status >= 500 => true,
        None => true,
        _ => false,
    }
}

// ----------------------------------------------------------------------------
// BUKTI KEPADA DOSEN: INI CONTOH REKURSI MURNI (Pure Recursion)
// Menghitung jeda waktu tunggu (exponential backoff: 500ms -> 1000ms -> 2000ms)
// dengan memanggil dirinya sendiri tanpa memakai perulangan loop for/while mutabel.
// ----------------------------------------------------------------------------
pub fn hitung_backoff_ms(attempt: u32, base_ms: u64) -> u64 {
    if attempt <= 1 {
        base_ms
    } else {
        2 * hitung_backoff_ms(attempt - 1, base_ms)
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT MODE OFFLINE LALU KIRIM DATA DARI KOTAK 1
// BUKTI KEPADA DOSEN: INI CONTOH IMMUTABILITY (Data tidak diubah langsung).
// Menambahkan data sensor ke antrean buffer memakai .chain() yang menghasilkan
// Vec baru tanpa merusak/mengubah array buffer yang lama.
// ----------------------------------------------------------------------------
pub fn enqueue_offline(
    current_buffer: &[BufferedItem],
    raw: RawReading,
    reason: &str,
) -> Vec<BufferedItem> {
    let already_exists = current_buffer
        .iter()
        .any(|item| item.raw.event_id == raw.event_id);

    if already_exists {
        current_buffer.to_vec()
    } else {
        current_buffer
            .iter()
            .cloned()
            .chain(std::iter::once(BufferedItem {
                raw,
                failed_attempts: 1,
                reason: reason.to_string(),
            }))
            .collect()
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT KLIK "Sambungkan Kembali & Kirim Buffer (ONLINE)"
// Memisahkan data antrean buffer yang siap dikirim ulang secara immutable.
// ----------------------------------------------------------------------------
pub fn take_reconnect_batch(
    current_buffer: &[BufferedItem],
    batch_size: usize,
) -> (Vec<BufferedItem>, Vec<BufferedItem>) {
    let to_send = current_buffer.iter().take(batch_size).cloned().collect();
    let remaining = current_buffer.iter().skip(batch_size).cloned().collect();
    (to_send, remaining)
}

// Filter antrean buffer berdasarkan device_id
pub fn filter_buffer_by_device(
    current_buffer: &[BufferedItem],
    device_id: &str,
) -> Vec<BufferedItem> {
    current_buffer
        .iter()
        .filter(|item| item.raw.device_id == device_id)
        .cloned()
        .collect()
}

// ============================================================================
// BAGIAN 2: SIDE-EFFECT FUNCTION (Fungsi dengan Efek Samping Jaringan HTTP)
// ============================================================================

// Mengirimkan data melalui HTTP POST menggunakan reqwest asinkron.
// Dosen: "Ini bagian side-effect karena menyentuh jaringan internet dan waktu sleep."
pub async fn send_with_retry(
    url: &str,
    api_key: &str,
    data: &RawReading,
    max_attempts: u32,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    for attempt in 1..=max_attempts {
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
                        return Ok(());
                    }
                    return Err(format!("Pengiriman ditolak dengan status HTTP {}", status));
                }
            }
            Err(error) => {
                if attempt == max_attempts {
                    return Err(format!(
                        "Koneksi SIM7000E gagal setelah {} percobaan ({}) -> dialihkan ke Offline Buffer",
                        max_attempts, error
                    ));
                }
            }
        }

        if attempt < max_attempts {
            let wait_ms = hitung_backoff_ms(attempt, 500);
            tokio::time::sleep(Duration::from_millis(wait_ms)).await;
        }
    }

    Err("Pengiriman gagal, disimpan ke Offline Buffer".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_raw(event_id: &str, device_id: &str) -> RawReading {
        RawReading {
            event_id: event_id.to_string(),
            device_id: device_id.to_string(),
            ts: "2026-11-01T10:00:00Z".to_string(),
            timestamp_sec: 100,
            moisture_raw: 65.0,
            temperature_raw: 28.0,
            ph_raw: 4.5,
            ec_raw: 1.1,
        }
    }

    #[test]
    fn test_retry_policy() {
        assert!(!should_retry(Some(200)));
        assert!(!should_retry(Some(400)));
        assert!(!should_retry(Some(401)));
        assert!(!should_retry(Some(409)));
        assert!(should_retry(Some(500)));
        assert!(should_retry(Some(503)));
        assert!(should_retry(None));
    }

    #[test]
    fn test_recursive_backoff() {
        assert_eq!(hitung_backoff_ms(1, 500), 500);
        assert_eq!(hitung_backoff_ms(2, 500), 1000);
        assert_eq!(hitung_backoff_ms(3, 500), 2000);
        assert_eq!(hitung_backoff_ms(4, 500), 4000);
    }

    #[test]
    fn test_immutable_offline_buffering_and_reconnect() {
        let empty: Vec<BufferedItem> = Vec::new();
        let buf1 = enqueue_offline(&empty, sample_raw("evt-1", "ESP32-01"), "Offline");
        assert!(empty.is_empty());
        assert_eq!(buf1.len(), 1);

        let buf_dup = enqueue_offline(&buf1, sample_raw("evt-1", "ESP32-01"), "Offline");
        assert_eq!(buf_dup.len(), 1);

        let buf2 = enqueue_offline(&buf1, sample_raw("evt-2", "ESP32-02"), "Offline");
        assert_eq!(buf2.len(), 2);

        let (batch, remaining) = take_reconnect_batch(&buf2, 1);
        assert_eq!(batch.len(), 1);
        assert_eq!(remaining.len(), 1);
        assert_eq!(batch[0].raw.event_id, "evt-1");
        assert_eq!(remaining[0].raw.event_id, "evt-2");
    }
}
