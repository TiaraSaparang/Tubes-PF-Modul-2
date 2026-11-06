// ============================================================================
// FILE: src/contracts.rs
// STATUS: PURE CONTRACTS (Struktur Data Antarmodul Tanpa Efek Samping)
// HUBUNGAN KE WEB UI: KOTAK 1 (Input Sensor) & KOTAK 4 (Output JSON ke Modul 3)
// ============================================================================

use serde::{Deserialize, Serialize};

// Parameter tanah minimum wajib sesuai RAB Modul 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Parameter {
    Moisture,
    Temperature,
    Ph,
    Ec,
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: KOLOM "Quality" PADA TABEL DI KOTAK 4
// - Good    : Sensor normal & tanah ideal
// - Suspect : Sensor normal tapi kondisi tanah ekstrem (misal kemarau)
// - Bad     : Sensor rusak / kabel putus (memicu Buzzer menyala)
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityFlag {
    Good,
    Suspect,
    Bad,
    Calibrating,
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DATA YANG DIKIRIMKAN DARI KOTAK 1 KE SERVER
// Format data mentah dari sensor ESP32 sebelum diproses oleh Modul 2.
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawReading {
    pub event_id: String,
    pub device_id: String,
    pub ts: String,
    pub timestamp_sec: u64,
    pub moisture_raw: f32,
    pub temperature_raw: f32,
    pub ph_raw: f32,
    pub ec_raw: f32,
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: TABEL DAN KOTAK PREVIEW JSON DI KOTAK 4
// Format data bersih tervalidasi yang siap diserahkan ke Modul 3 (ReadingSink).
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoilReading {
    pub device_id: String,
    pub plot_id: String,
    pub ts: String,
    pub moisture_pct: f32,
    pub temperature_c: f32,
    pub ph: f32,
    pub ec_ms_cm: f32,
    pub quality: QualityFlag,
}

impl SoilReading {
    pub fn is_usable(&self) -> bool {
        matches!(self.quality, QualityFlag::Good | QualityFlag::Suspect)
    }

    pub fn value(&self, param: Parameter) -> f32 {
        match param {
            Parameter::Moisture => self.moisture_pct,
            Parameter::Temperature => self.temperature_c,
            Parameter::Ph => self.ph,
            Parameter::Ec => self.ec_ms_cm,
        }
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: INFO DETAIL LAHAN PADA TABEL KOTAK 3 DAN KOTAK 4
// Permintaan khusus dari Lisa (Modul 3): nama petak, koordinat GPS, luas, tanaman.
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlotMeta {
    pub plot_id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub area_ha: f32,
    pub plant: String,
}

// Paket lengkap gabungan data sensor + info kebun + status buzzer untuk Modul 3
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M3IngestEnvelope {
    pub reading: SoilReading,
    pub plot_meta: PlotMeta,
    pub owner_username: String,
    pub anomalies: Vec<String>,
    pub buzzer_alarm_active: bool,
}

// Trait pintu masuk Modul 3 (ReadingSink) yang dipanggil oleh Modul 2
pub trait ReadingSink: Send + Sync {
    fn ingest(&self, reading: SoilReading, plot_meta: &PlotMeta) -> Result<(), String>;
}
