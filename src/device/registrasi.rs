//! Modul registrasi perangkat.

use super::model::Device;
use std::collections::HashMap;

/// Registry penyimpan perangkat terdaftar.
pub struct Registry {
    devices: HashMap<String, Device>,
}

impl Registry {
    /// Membuat Registry kosong.
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    /// Mendaftarkan perangkat baru.
    pub fn registrasi(&mut self, device: Device) -> Result<(), String> {
        if self.devices.contains_key(&device.id) {
            return Err(format!("Perangkat dengan ID '{}' sudah terdaftar", device.id));
        }
        self.devices.insert(device.id.clone(), device);
        Ok(())
    }

    /// Mencari perangkat berdasarkan ID.
    pub fn cari(&self, id: &str) -> Option<&Device> {
        self.devices.get(id)
    }

    /// Menghapus perangkat.
    pub fn hapus(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    /// Jumlah perangkat terdaftar.
    pub fn jumlah(&self) -> usize {
        self.devices.len()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}