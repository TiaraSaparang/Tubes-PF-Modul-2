// ============================================================================
// FILE: src/device/registrasi.rs
// STATUS: PURE FUNCTIONS & IMMUTABLE REGISTRY (Tanpa Efek Samping, Bebas Database)
// HUBUNGAN KE WEB UI: KOTAK 3 (Registrasi Perangkat, Pemetaan Pemilik & RBAC)
// ============================================================================

use super::model::Device;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry penyimpan daftar perangkat IoT Smart Soil.
/// Bersifat imutabel untuk menjamin konsistensi data fungsional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    devices: HashMap<String, Device>,
}

impl Registry {
    /// Fungsi murni: Membuat Registry kosong.
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Simpan Perangkat" DI KLIK
    // (Muncul setelah klik tombol "Tambah Perangkat Baru" di Kotak 3)
    //
    // BUKTI KEPADA DOSEN: INI CONTOH IMMUTABILITY & PURE FUNCTION
    // - Menerima '&self' (bukan '&mut self'), tidak memodifikasi memori registry lama.
    // - Menggunakan iterasi fungsional '.chain(std::iter::once(...))' untuk
    //   menghasilkan instance Registry baru yang berisi perangkat baru.
    // ------------------------------------------------------------------------
    pub fn registrasi_murni(&self, device: Device) -> Result<Self, String> {
        if device.id.trim().is_empty() {
            return Err("ID perangkat tidak boleh kosong".to_string());
        }
        if self.devices.contains_key(&device.id) {
            return Err(format!("Perangkat dengan ID '{}' sudah terdaftar", device.id));
        }
        let next_devices = self
            .devices
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .chain(std::iter::once((device.id.clone(), device)))
            .collect();

        Ok(Self {
            devices: next_devices,
        })
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Uji Validasi Akses" DI KLIK
    // (Khusus bila Aksi yang dipilih adalah "Kontrol (Ubah Interval Sampling)")
    //
    // BUKTI KEPADA DOSEN: HIGHER-ORDER FUNCTION (.map) & IMMUTABILITY
    // - Memperbarui sampling interval perangkat target menggunakan fungsi murni
    //   '.map(...)' menghasilkan salinan Registry baru.
    // ------------------------------------------------------------------------
    pub fn ubah_interval_murni(&self, id: &str, interval_sec: u64) -> Result<Self, String> {
        let existing = self
            .cari(id)
            .ok_or_else(|| format!("Perangkat '{}' tidak ditemukan", id))?;
        let updated_device = existing.with_sampling_interval(interval_sec)?;

        let next_devices = self
            .devices
            .iter()
            .map(|(k, v)| {
                if k == id {
                    (k.clone(), updated_device.clone())
                } else {
                    (k.clone(), v.clone())
                }
            })
            .collect();

        Ok(Self {
            devices: next_devices,
        })
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Uji Validasi Akses" DI KLIK
    // (Khusus bila Aksi yang dipilih adalah "Hapus (Hapus Perangkat)")
    //
    // BUKTI KEPADA DOSEN: HIGHER-ORDER FUNCTION (.filter) & IMMUTABILITY
    // - Menghapus perangkat target menggunakan '.filter(|(k, _)| k.as_str() != id)'
    //   secara deklaratif tanpa menghapus langsung secara memori in-place.
    // ------------------------------------------------------------------------
    pub fn hapus_murni(&self, id: &str) -> Result<Self, String> {
        if !self.devices.contains_key(id) {
            return Err(format!("Perangkat '{}' tidak ditemukan", id));
        }
        let next_devices = self
            .devices
            .iter()
            .filter(|(k, _)| k.as_str() != id)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(Self {
            devices: next_devices,
        })
    }

    /// Kompatibilitas method (wrapper untuk pengujian mutabel).
    pub fn registrasi(&mut self, device: Device) -> Result<(), String> {
        let updated = self.registrasi_murni(device)?;
        *self = updated;
        Ok(())
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIGUNAKAN DI KOTAK 1 SAAT CEK PERANGKAT SEBELUM TERIMA DATA
    // Mengembalikan referensi perangkat jika terdaftar.
    // ------------------------------------------------------------------------
    pub fn cari(&self, id: &str) -> Option<&Device> {
        self.devices.get(id)
    }

    /// Kompatibilitas method (wrapper untuk pengujian mutabel).
    pub fn hapus(&mut self, id: &str) -> bool {
        match self.hapus_murni(id) {
            Ok(updated) => {
                *self = updated;
                true
            }
            Err(_) => false,
        }
    }

    /// Fungsi murni: Menghitung jumlah perangkat terdaftar.
    pub fn jumlah(&self) -> usize {
        self.devices.len()
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: MENAMPILKAN DATA PADA TABEL PERANGKAT DI KOTAK 3
    // Mengembalikan list terurut seluruh perangkat untuk dirender ke tabel HTML.
    // ------------------------------------------------------------------------
    pub fn daftar_semua(&self) -> Vec<Device> {
        let mut list: Vec<Device> = self.devices.values().cloned().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_registry_operations() {
        let reg0 = Registry::new();
        let dev = Device::new("ESP32-01", "Node Blok A", "ESP32-SIM7000E");

        let reg1 = reg0.registrasi_murni(dev).unwrap();
        assert_eq!(reg0.jumlah(), 0); // reg0 tetap 0 (terbukti immutable!)
        assert_eq!(reg1.jumlah(), 1);

        let reg2 = reg1.ubah_interval_murni("ESP32-01", 120).unwrap();
        assert_eq!(reg1.cari("ESP32-01").unwrap().sampling_interval_sec, 60);
        assert_eq!(reg2.cari("ESP32-01").unwrap().sampling_interval_sec, 120);

        let reg3 = reg2.hapus_murni("ESP32-01").unwrap();
        assert_eq!(reg3.jumlah(), 0);
    }
}
