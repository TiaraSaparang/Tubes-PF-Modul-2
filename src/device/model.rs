// ============================================================================
// FILE: src/device/model.rs
// STATUS: PURE STRUCT (Model Perangkat IoT Tanpa Efek Samping)
// HUBUNGAN KE WEB UI: KOTAK 3 (Daftar ID Perangkat, Tipe Hardware, & Interval)
// ============================================================================

use serde::{Deserialize, Serialize};

// Representasi identitas alat mikrokontroler (ESP32 + SIM7000E) di lapangan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub nama: String,
    pub tipe: String,
    pub sampling_interval_sec: u64,
}

impl Device {
    pub fn new(id: &str, nama: &str, tipe: &str) -> Self {
        Self {
            id: id.to_string(),
            nama: nama.to_string(),
            tipe: tipe.to_string(),
            sampling_interval_sec: 60,
        }
    }

    // ------------------------------------------------------------------------
    // BUKTI KEPADA DOSEN: IMMUTABLE STRUCT UPDATE
    // Menghasilkan instance Device baru dengan interval baru tanpa memutasi (&mut) yang lama.
    // ------------------------------------------------------------------------
    pub fn with_sampling_interval(&self, interval_sec: u64) -> Result<Self, String> {
        if !(5..=86_400).contains(&interval_sec) {
            return Err("Interval sampling harus berada di antara 5 hingga 86400 detik".to_string());
        }
        Ok(Self {
            sampling_interval_sec: interval_sec,
            ..self.clone()
        })
    }
}
