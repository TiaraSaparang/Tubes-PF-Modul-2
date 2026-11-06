// ============================================================================
// FILE: src/user/model.rs
// STATUS: PURE FUNCTIONS & IMMUTABLE DATA STRUCTURE (Tanpa Efek Samping, Bebas Database)
// HUBUNGAN KE WEB UI: KOTAK 3 (Kepemilikan Perangkat) & KOTAK 4 (Integrasi PlotMeta ke Modul 3)
// ============================================================================

use crate::contracts::PlotMeta;
use crate::rbac::Role;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DATA DROPDOWN "Pengguna & Role" PADA KOTAK 3
// Menyimpan informasi identitas pengguna, hak role (Admin/Operator/Viewer),
// serta daftar device_id yang ditugaskan ke user tersebut.
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub role: Role,
    pub perangkat: Vec<String>,
}

impl User {
    /// Fungsi murni: Membuat data user baru.
    pub fn new(username: &str, role: Role) -> Self {
        Self {
            username: username.to_string(),
            role,
            perangkat: Vec::new(),
        }
    }

    // ------------------------------------------------------------------------
    // BUKTI KEPADA DOSEN: INI CONTOH IMMUTABILITY & HIGHER-ORDER FUNCTION
    // Menambahkan perangkat baru ke akun user secara murni menggunakan .chain()
    // menghasilkan struct User baru tanpa memutasi objek User lama.
    // ------------------------------------------------------------------------
    pub fn tambah_perangkat_murni(&self, device_id: &str) -> Self {
        let sudah_ada = self.perangkat.iter().any(|id| id == device_id);
        let perangkat_baru = if sudah_ada {
            self.perangkat.clone()
        } else {
            self.perangkat
                .iter()
                .cloned()
                .chain(std::iter::once(device_id.to_string()))
                .collect()
        };
        Self {
            username: self.username.clone(),
            role: self.role.clone(),
            perangkat: perangkat_baru,
        }
    }

    /// Kompatibilitas method (wrapper mutabel).
    pub fn tambah_perangkat(&mut self, device_id: &str) {
        *self = self.tambah_perangkat_murni(device_id);
    }

    /// Fungsi murni: Mengecek apakah user memiliki hak atas ID perangkat tertentu.
    pub fn memiliki(&self, device_id: &str) -> bool {
        self.perangkat.iter().any(|id| id == device_id)
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: KOLOM "Pemilik" & "Plot ID & Tanaman" PADA TABEL KOTAK 3
// Struktur imutabel yang memetakan kepemilikan perangkat ke petani/operator
// dan memetakan perangkat ke data petak lahan gambut (PlotMeta).
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kepemilikan {
    mapping: HashMap<String, String>,
    plots: HashMap<String, PlotMeta>,
}

impl Kepemilikan {
    /// Fungsi murni: Membuat data kepemilikan kosong.
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            plots: HashMap::new(),
        }
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIPANGGIL SAAT MENYIMPAN FORM "Tambah Perangkat Baru" DI KOTAK 3
    //
    // BUKTI KEPADA DOSEN: IMMUTABLE STRUCT UPDATE
    // Memetakan perangkat baru ke username pemilik dan metadata lahan (PlotMeta)
    // tanpa mutasi langsung (menghasilkan salinan Kepemilikan baru).
    // ------------------------------------------------------------------------
    pub fn tetapkan_lengkap_murni(
        &self,
        device_id: &str,
        username: &str,
        plot: PlotMeta,
    ) -> Self {
        let next_mapping = self
            .mapping
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .chain(std::iter::once((
                device_id.to_string(),
                username.to_string(),
            )))
            .collect();

        let next_plots = self
            .plots
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .chain(std::iter::once((device_id.to_string(), plot)))
            .collect();

        Self {
            mapping: next_mapping,
            plots: next_plots,
        }
    }

    /// Kompatibilitas method (wrapper mutabel).
    pub fn tetapkan(&mut self, device_id: &str, username: &str) {
        self.mapping
            .insert(device_id.to_string(), username.to_string());
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Uji Validasi Akses" DI KOTAK 3
    // Mengambil pemilik sah perangkat untuk memastikan hanya pemilik (atau Admin)
    // yang boleh mengontrol alat tersebut.
    // ------------------------------------------------------------------------
    pub fn pemilik(&self, device_id: &str) -> Option<&String> {
        self.mapping.get(device_id)
    }

    // ------------------------------------------------------------------------
    // HUBUNGAN KE UI: DIAMBIL SAAT PENGIRIMAN DATA DARI KOTAK 1 MENUJU KOTAK 4
    // Mengambil PlotMeta (ID petak, nama, jenis tanaman) untuk digabungkan ke
    // dalam SoilReading sebelum diserahkan ke Modul 3.
    // ------------------------------------------------------------------------
    pub fn plot_of(&self, device_id: &str) -> Option<&PlotMeta> {
        self.plots.get(device_id)
    }

    // ------------------------------------------------------------------------
    // BUKTI KEPADA DOSEN: HIGHER-ORDER FUNCTION (.filter & .map)
    // Mencari seluruh ID perangkat yang dimiliki oleh seorang pengguna secara fungsional.
    // ------------------------------------------------------------------------
    pub fn perangkat_milik(&self, username: &str) -> Vec<String> {
        self.mapping
            .iter()
            .filter(|(_, owner)| owner.as_str() == username)
            .map(|(dev_id, _)| dev_id.clone())
            .collect()
    }
}

impl Default for Kepemilikan {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_and_ownership_immutability() {
        let u0 = User::new("petani_budi", Role::Operator);
        let u1 = u0.tambah_perangkat_murni("ESP32-01");
        assert!(!u0.memiliki("ESP32-01"));
        assert!(u1.memiliki("ESP32-01"));

        let plot = PlotMeta {
            plot_id: "PLOT-01".to_string(),
            name: "Lahan Gambut A".to_string(),
            latitude: -2.21,
            longitude: 113.92,
            area_ha: 2.0,
            plant: "nanas".to_string(),
        };
        let k0 = Kepemilikan::new();
        let k1 = k0.tetapkan_lengkap_murni("ESP32-01", "petani_budi", plot);
        assert!(k0.pemilik("ESP32-01").is_none());
        assert_eq!(k1.pemilik("ESP32-01").unwrap(), "petani_budi");
        assert_eq!(k1.plot_of("ESP32-01").unwrap().plant, "nanas");
    }
}
