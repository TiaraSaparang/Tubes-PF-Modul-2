/// Mengecek apakah suhu melebihi batas yang ditentukan.
/// 
/// Jika suhu lebih dari 80.0 °C, maka dikategorikan sebagai ANOMALY.
/// Jika tidak, maka dikategorikan sebagai NORMAL.
pub fn check_temperature(temperature: f64) -> &'static str {
    if temperature > 80.0 {
        "ANOMALY"
    } else {
        "NORMAL"
    }
}

/// Mengecek apakah tekanan melebihi batas yang ditentukan.
///
/// Jika tekanan lebih dari 100.0, maka dikategorikan sebagai ANOMALY.
/// Jika tidak, maka dikategorikan sebagai NORMAL.
pub fn check_pressure(pressure: f64) -> &'static str {
    if pressure > 100.0 {
        "ANOMALY"
    } else {
        "NORMAL"
    }
}

/// Mengecek apakah getaran melebihi batas yang ditentukan.
///
/// Jika getaran lebih dari 50.0, maka dikategorikan sebagai ANOMALY.
/// Jika tidak, maka dikategorikan sebagai NORMAL.
pub fn check_vibration(vibration: f64) -> &'static str {
    if vibration > 50.0 {
        "ANOMALY"
    } else {
        "NORMAL"
    }
}

/// Mengecek apakah interval sampling sudah terpenuhi.
///
/// Mengembalikan `true` jika waktu sekarang dikurangi waktu
/// sampling sebelumnya sudah mencapai interval yang ditentukan.
pub fn check_interval(
    current_time: u64,
    last_time: u64,
    interval: u64,
) -> bool {
    current_time - last_time >= interval
}

/// Menyimpan data yang diterima dari sensor.
///
/// Data yang disimpan terdiri dari suhu, tekanan, dan getaran.
pub struct SensorData {
    pub temperature: f64,
    pub pressure: f64,
    pub vibration: f64,
}

/// Memeriksa seluruh data sensor untuk mendeteksi anomaly.
///
/// Fungsi ini akan memeriksa suhu, tekanan, dan getaran.
/// Jika terdapat nilai yang melewati batas, informasi anomaly
/// akan dimasukkan ke dalam hasil.
pub fn check_anomaly(data: &SensorData) -> Vec<String> {
    let mut results = Vec::new();

    if check_temperature(data.temperature) == "ANOMALY" {
        results.push("Temperature exceeds limit".to_string());
    }

    if check_pressure(data.pressure) == "ANOMALY" {
        results.push("Pressure exceeds limit".to_string());
    }

    if check_vibration(data.vibration) == "ANOMALY" {
        results.push("Vibration exceeds limit".to_string());
    }

    results
}


// ============================================
// UNIT TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_normal() {
        assert_eq!(check_temperature(75.0), "NORMAL");
    }

    #[test]
    fn test_temperature_anomaly() {
        assert_eq!(check_temperature(85.0), "ANOMALY");
    }

    #[test]
    fn test_pressure_normal() {
        assert_eq!(check_pressure(90.0), "NORMAL");
    }

    #[test]
    fn test_pressure_anomaly() {
        assert_eq!(check_pressure(110.0), "ANOMALY");
    }

    #[test]
    fn test_vibration_normal() {
        assert_eq!(check_vibration(30.0), "NORMAL");
    }

    #[test]
    fn test_vibration_anomaly() {
        assert_eq!(check_vibration(60.0), "ANOMALY");
    }

    #[test]
    fn test_interval_reached() {
        assert_eq!(check_interval(30, 20, 10), true);
    }

    #[test]
    fn test_interval_not_reached() {
        assert_eq!(check_interval(25, 20, 10), false);
    }

    #[test]
    fn test_all_sensor_normal() {
        let data = SensorData {
            temperature: 75.0,
            pressure: 90.0,
            vibration: 30.0,
        };

        let results = check_anomaly(&data);

        assert!(results.is_empty());
    }

    #[test]
    fn test_sensor_anomaly() {
        let data = SensorData {
            temperature: 85.0,
            pressure: 110.0,
            vibration: 60.0,
        };

        let results = check_anomaly(&data);

        assert_eq!(results.len(), 3);
    }
}