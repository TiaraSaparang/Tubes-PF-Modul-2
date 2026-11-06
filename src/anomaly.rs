// ============================================================================
// FILE: src/anomaly.rs
// STATUS: 100% PURE FUNCTIONS (Fungsi Murni, Imutabel, Tanpa Efek Samping, Bebas I/O)
// HUBUNGAN KE WEB UI: KOTAK 1 (Input Data Sensor Edge & Kontrol Interval)
//                     & STATUS BAR ATAS (Status Buzzer 5V: MATI / MENYALA)
// ============================================================================
//
// PENJELASAN KEPADA DOSEN (ARSITEKTUR PEMROGRAMAN FUNGSIONAL):
// Seluruh fungsi di file ini adalah PURE FUNCTIONS (Fungsi Murni):
// 1. Tidak ada efek samping (no side-effects): Tidak mengakses database, jaringan,
//    atau menyalakan hardware langsung. Semua kalkulasi murni di memori.
// 2. Deterministik & Imutabel: Input yang sama selalu menghasilkan output yang sama persis.
// 3. Bukti Fungsional: Menggunakan Higher-Order Function (.filter_map, .all, .any)
//    dan Function Composition (.and_then, .map) tanpa loop for/while mutabel.
//
// DAFTAR 4 SENSOR WAJIB (SESUAI RAB MODUL 1):
// 1. Kelembapan Tanah (moisture_pct) : DFRobot Capacitive Moisture + ADC ADS1115
// 2. Suhu Tanah (temperature_c)      : Waterproof Probe DS18B20
// 3. pH Tanah (ph)                   : Sensor pH Analog Tanah + DMS
// 4. Konduktivitas / EC (ec_ms_cm)   : DFRobot Gravity Analog TDS/EC Sensor V1.0
// 5. Output Fisik                    : Buzzer Aktif 5V (Menyala saat kualitas BAD / kabel putus)
// ============================================================================

use crate::contracts::{PlotMeta, QualityFlag, RawReading, SoilReading};
use serde::{Deserialize, Serialize};

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: MENENTUKAN NILAI PADA KOLOM "Quality" DI KOTAK 4
// - Normal          -> Menghasilkan QualityFlag::Good    (Kondisi tanah gambut optimal)
// - Warning         -> Menghasilkan QualityFlag::Suspect (Tanah gambut kering/ekstrem)
// - CriticalAnomaly -> Menghasilkan QualityFlag::Bad     (Sensor rusak / kabel putus)
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorStatus {
    Normal,
    Warning,
    CriticalAnomaly,
}

// ============================================================================
// BAGIAN 1: PURE FUNCTIONS - VALIDASI INTERVAL SAMPLING
// ============================================================================

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Uji Interval Belum Tercapai" DI-KLIK
//
// BUKTI KEPADA DOSEN: PURE FUNCTION (MATEMATIKA DETERMINISTIK)
// - Menghitung apakah jeda waktu (current_time - last_time) sudah mencapai interval.
// - Jika belum tercapai, sistem menolak pengiriman dengan status HTTP 429.
// - TUJUAN SISTEM: Menghemat daya baterai Li-Ion 18650 (3x) dan kuota modem SIM7000E.
// ----------------------------------------------------------------------------
pub fn check_interval(current_time: u64, last_time: u64, interval: u64) -> bool {
    current_time.saturating_sub(last_time) >= interval
}

// ============================================================================
// BAGIAN 2: PURE FUNCTIONS - KLASIFIKASI 4 SENSOR TANAH GAMBUT (RAB M1)
// ============================================================================

// ----------------------------------------------------------------------------
// SENSOR 1: KELEMBAPAN TANAH (DFRobot Capacitive Soil Moisture Sensor)
// - Batas Fisik Alat  : 0.0% ..= 100.0%  (Di luar ini = Sensor Rusak / Kabel Putus)
// - Rentang Ideal Gambut : 30.0% ..= 90.0%  (Di bawah 30% = Gambut Kering Rawan Terbakar)
// ----------------------------------------------------------------------------
pub fn check_moisture(moisture_pct: f32) -> &'static str {
    match classify_moisture(moisture_pct) {
        SensorStatus::Normal => "NORMAL",
        SensorStatus::Warning | SensorStatus::CriticalAnomaly => "ANOMALY",
    }
}

pub fn classify_moisture(moisture_pct: f32) -> SensorStatus {
    if !moisture_pct.is_finite() || !(0.0..=100.0).contains(&moisture_pct) {
        SensorStatus::CriticalAnomaly
    } else if !(30.0..=90.0).contains(&moisture_pct) {
        SensorStatus::Warning
    } else {
        SensorStatus::Normal
    }
}

// ----------------------------------------------------------------------------
// SENSOR 2: SUHU TANAH (DS18B20 Waterproof Probe)
// - Batas Fisik Alat  : -10.0 C ..= 80.0 C (Di luar ini = Sensor Rusak)
// - Rentang Ideal Gambut : 18.0 C ..= 40.0 C  (Di luar ini = Suhu Ekstrem)
// ----------------------------------------------------------------------------
pub fn check_temperature(temperature_c: f32) -> &'static str {
    match classify_temperature(temperature_c) {
        SensorStatus::Normal => "NORMAL",
        SensorStatus::Warning | SensorStatus::CriticalAnomaly => "ANOMALY",
    }
}

pub fn classify_temperature(temperature_c: f32) -> SensorStatus {
    if !temperature_c.is_finite() || !(-10.0..=80.0).contains(&temperature_c) {
        SensorStatus::CriticalAnomaly
    } else if !(18.0..=40.0).contains(&temperature_c) {
        SensorStatus::Warning
    } else {
        SensorStatus::Normal
    }
}

// ----------------------------------------------------------------------------
// SENSOR 3: pH TANAH GAMBUT (Sensor pH Analog + DMS)
// - Batas Fisik Alat  : 0.0 ..= 14.0 (Skala pH Dunia)
// - Rentang Wajar Gambut : 3.0 ..= 7.0  (Gambut alami sangat masam s/d pasca-kapur)
// ----------------------------------------------------------------------------
pub fn check_ph(ph: f32) -> &'static str {
    match classify_ph(ph) {
        SensorStatus::Normal => "NORMAL",
        SensorStatus::Warning | SensorStatus::CriticalAnomaly => "ANOMALY",
    }
}

pub fn classify_ph(ph: f32) -> SensorStatus {
    if !ph.is_finite() || !(0.0..=14.0).contains(&ph) {
        SensorStatus::CriticalAnomaly
    } else if !(3.0..=7.0).contains(&ph) {
        SensorStatus::Warning
    } else {
        SensorStatus::Normal
    }
}

// ----------------------------------------------------------------------------
// SENSOR 4: DAYA HANTAR LISTRIK / EC (DFRobot Gravity Analog TDS/EC Sensor)
// - Batas Fisik Alat  : 0.0 ..= 20.0 mS/cm
// - Rentang Wajar Gambut : 0.05 ..= 4.0 mS/cm (Kadar salinitas & kesuburan pupuk)
// ----------------------------------------------------------------------------
pub fn check_ec(ec_ms_cm: f32) -> &'static str {
    match classify_ec(ec_ms_cm) {
        SensorStatus::Normal => "NORMAL",
        SensorStatus::Warning | SensorStatus::CriticalAnomaly => "ANOMALY",
    }
}

pub fn classify_ec(ec_ms_cm: f32) -> SensorStatus {
    if !ec_ms_cm.is_finite() || !(0.0..=20.0).contains(&ec_ms_cm) {
        SensorStatus::CriticalAnomaly
    } else if !(0.05..=4.0).contains(&ec_ms_cm) {
        SensorStatus::Warning
    } else {
        SensorStatus::Normal
    }
}

// ============================================================================
// BAGIAN 3: PURE HIGHER-ORDER FUNCTION (EVALUASI DAFTAR ATURAN ATURAN ANOMALI)
// ============================================================================

/// Tipe penunjuk fungsi aturan anomali (Higher-Order Function Callback)
pub type RuleFn = fn(&RawReading) -> Option<String>;

// ----------------------------------------------------------------------------
// BUKTI KEPADA DOSEN: INI HIGHER-ORDER FUNCTION (HOF)
// Fungsi ini menerima irisan fungsi aturan (&[RuleFn]) lalu memprosesnya
// menggunakan metode fungsional murni '.iter().filter_map(...)'.
// Tidak ada variabel 'mut' dan tidak ada perulangan 'for loop' manual.
// ----------------------------------------------------------------------------
pub fn evaluate_with_rules(raw: &RawReading, rules: &[RuleFn]) -> Vec<String> {
    rules.iter().filter_map(|rule_fn| rule_fn(raw)).collect()
}

// Aturan 1: Deteksi Kelembapan Gambut
pub fn rule_moisture(raw: &RawReading) -> Option<String> {
    match classify_moisture(raw.moisture_raw) {
        SensorStatus::Normal => None,
        SensorStatus::Warning => Some(format!(
            "Anomali Kelembapan Gambut: {:.1}% di luar rentang ideal (30-90%)",
            raw.moisture_raw
        )),
        SensorStatus::CriticalAnomaly => Some(format!(
            "KRITIS Sensor Moisture (DFRobot/ADS1115): nilai {:.1}% tidak valid / indikasi kabel putus!",
            raw.moisture_raw
        )),
    }
}

// Aturan 2: Deteksi Suhu Tanah
pub fn rule_temperature(raw: &RawReading) -> Option<String> {
    match classify_temperature(raw.temperature_raw) {
        SensorStatus::Normal => None,
        SensorStatus::Warning => Some(format!(
            "Anomali Suhu Tanah: {:.1}°C di luar rentang normal gambut (18-40°C)",
            raw.temperature_raw
        )),
        SensorStatus::CriticalAnomaly => Some(format!(
            "KRITIS Sensor Suhu (DS18B20): nilai {:.1}°C di luar batas fisik (-10..80°C)!",
            raw.temperature_raw
        )),
    }
}

// Aturan 3: Deteksi pH Tanah Gambut
pub fn rule_ph(raw: &RawReading) -> Option<String> {
    match classify_ph(raw.ph_raw) {
        SensorStatus::Normal => None,
        SensorStatus::Warning => Some(format!(
            "Anomali pH Gambut: {:.2} di luar rentang wajar (3.0-7.0)",
            raw.ph_raw
        )),
        SensorStatus::CriticalAnomaly => Some(format!(
            "KRITIS Sensor pH (+DMS): nilai {:.2} di luar skala pH (0-14)!",
            raw.ph_raw
        )),
    }
}

// Aturan 4: Deteksi Konduktivitas EC
pub fn rule_ec(raw: &RawReading) -> Option<String> {
    match classify_ec(raw.ec_raw) {
        SensorStatus::Normal => None,
        SensorStatus::Warning => Some(format!(
            "Anomali EC Tanah: {:.2} mS/cm di luar rentang wajar gambut (0.05-4.0 mS/cm)",
            raw.ec_raw
        )),
        SensorStatus::CriticalAnomaly => Some(format!(
            "KRITIS Sensor EC (DFRobot): nilai {:.2} mS/cm di luar batas fisik (0-20 mS/cm)!",
            raw.ec_raw
        )),
    }
}

// Array konstan 4 aturan sensor tanah gambut
pub const STANDARD_PEATLAND_RULES: [RuleFn; 4] = [
    rule_moisture,
    rule_temperature,
    rule_ph,
    rule_ec,
];

// Menjalankan evaluasi 4 aturan anomali sensor secara deklaratif
pub fn check_anomaly(raw: &RawReading) -> Vec<String> {
    evaluate_with_rules(raw, &STANDARD_PEATLAND_RULES)
}

// ============================================================================
// BAGIAN 4: PURE FUNCTIONS - PENENTUAN STATUS QUALITY & BUZZER ALARM 5V
// ============================================================================

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Isi Data Normal / Kering / Rusak" DI-KLIK
// Menentukan flag kualitas data:
// 1. Jika ada sensor CriticalAnomaly -> QualityFlag::Bad
// 2. Jika ada sensor Warning         -> QualityFlag::Suspect
// 3. Jika semua normal               -> QualityFlag::Good
// ----------------------------------------------------------------------------
pub fn determine_quality(raw: &RawReading) -> QualityFlag {
    let statuses = [
        classify_moisture(raw.moisture_raw),
        classify_temperature(raw.temperature_raw),
        classify_ph(raw.ph_raw),
        classify_ec(raw.ec_raw),
    ];

    if statuses.iter().any(|s| *s == SensorStatus::CriticalAnomaly) {
        QualityFlag::Bad
    } else if statuses.iter().any(|s| *s == SensorStatus::Warning) {
        QualityFlag::Suspect
    } else {
        QualityFlag::Good
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: MENGUBAH TULISAN "Status Buzzer 5V" DI STATUS BAR ATAS
// (Aktif jika tombol "Isi Data Sensor Rusak (Bad)" di-klik lalu dikirim)
//
// BUKTI KEPADA DOSEN: PURE PREDICATE FUNCTION
// Hanya mengembalikan boolean true jika QualityFlag::Bad (indikasi kabel putus).
// Sinyal ini yang kemudian menginstruksikan hardware buzzer 5V untuk berbunyi.
// ----------------------------------------------------------------------------
pub fn should_trigger_buzzer(quality: QualityFlag) -> bool {
    matches!(quality, QualityFlag::Bad)
}

// ============================================================================
// BAGIAN 5: PURE FUNCTIONS - VALIDASI DASAR & KOMPOSISI TRANSFORMASI DATA
// ============================================================================

// Validasi dasar 1: Memastikan identitas event_id dan device_id tidak kosong
pub fn validate_identity_fields(raw: &RawReading) -> Result<&RawReading, String> {
    if raw.event_id.trim().is_empty() {
        Err("event_id tidak boleh kosong".to_string())
    } else if raw.device_id.trim().is_empty() {
        Err("device_id tidak boleh kosong".to_string())
    } else {
        Ok(raw)
    }
}

// Validasi dasar 2: Memastikan angka bukan NaN atau Infinity
pub fn validate_finite_numbers(raw: &RawReading) -> Result<&RawReading, String> {
    let values = [
        raw.moisture_raw,
        raw.temperature_raw,
        raw.ph_raw,
        raw.ec_raw,
    ];
    if values.iter().all(|v| v.is_finite()) {
        Ok(raw)
    } else {
        Err("Nilai sensor mengandung NaN atau Infinity (kabel sensor terputus)".to_string())
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Kirim Data Sensor (POST)" DI KOTAK 1
//
// BUKTI KEPADA DOSEN: FUNCTION COMPOSITION (.and_then & .map)
// Menggabungkan rangkaian validasi fungsional:
// 1. validate_identity_fields
// 2. .and_then(validate_finite_numbers)
// 3. .map(...) membentuk SoilReading bersih untuk diteruskan ke Modul 3.
// Jika salah satu gagal, langsung mengembalikan Err tanpa melanjutkan eksekusi.
// ----------------------------------------------------------------------------
pub fn validate_and_transform(raw: &RawReading, plot: &PlotMeta) -> Result<SoilReading, String> {
    validate_identity_fields(raw)
        .and_then(validate_finite_numbers)
        .map(|valid_raw| SoilReading {
            device_id: valid_raw.device_id.clone(),
            plot_id: plot.plot_id.clone(),
            ts: valid_raw.ts.clone(),
            moisture_pct: valid_raw.moisture_raw,
            temperature_c: valid_raw.temperature_raw,
            ph: valid_raw.ph_raw,
            ec_ms_cm: valid_raw.ec_raw,
            quality: determine_quality(valid_raw),
        })
}

// ============================================================================
// UNIT TESTS (PENGUJIAN OTOMATIS)
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_raw(moisture: f32, temp: f32, ph: f32, ec: f32) -> RawReading {
        RawReading {
            event_id: "evt-001".to_string(),
            device_id: "ESP32-GAMBUT-01".to_string(),
            ts: "2026-11-01T10:00:00Z".to_string(),
            timestamp_sec: 100,
            moisture_raw: moisture,
            temperature_raw: temp,
            ph_raw: ph,
            ec_raw: ec,
        }
    }

    fn sample_plot() -> PlotMeta {
        PlotMeta {
            plot_id: "PLOT-KALIMANTAN-01".to_string(),
            name: "Petak Nanas Gambut Blok A".to_string(),
            latitude: -2.2083,
            longitude: 113.9167,
            area_ha: 1.5,
            plant: "nanas".to_string(),
        }
    }

    #[test]
    fn test_all_peatland_sensors_normal() {
        let raw = sample_raw(68.5, 28.4, 4.5, 1.2);
        let anomalies = check_anomaly(&raw);
        assert!(anomalies.is_empty());
        assert_eq!(determine_quality(&raw), QualityFlag::Good);
        assert!(!should_trigger_buzzer(determine_quality(&raw)));
    }

    #[test]
    fn test_warning_creates_suspect_quality() {
        let raw = sample_raw(15.0, 28.4, 4.5, 1.2);
        let anomalies = check_anomaly(&raw);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(determine_quality(&raw), QualityFlag::Suspect);
        assert!(!should_trigger_buzzer(determine_quality(&raw)));
    }

    #[test]
    fn test_critical_sensor_triggers_bad_quality_and_buzzer() {
        let raw = sample_raw(145.0, 28.4, -2.0, 1.2);
        let reading = validate_and_transform(&raw, &sample_plot()).unwrap();
        assert_eq!(reading.quality, QualityFlag::Bad);
        assert!(should_trigger_buzzer(reading.quality));
    }

    #[test]
    fn test_interval_check_pure() {
        assert!(check_interval(300, 240, 60));
        assert!(!check_interval(280, 240, 60));
    }
}
