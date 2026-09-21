//! Modul model pengguna & kepemilikan perangkat.

use crate::device::Device;
use crate::rbac::Role;
use std::collections::HashMap;

/// Representasi pengguna dalam sistem.
#[derive(Debug, Clone)]
pub struct User {
    /// Username unik pengguna.
    pub username: String,
    /// Role pengguna (RBAC).
    pub role: Role,
    /// Daftar ID perangkat yang dimiliki user ini.
    pub perangkat: Vec<String>,
}

impl User {
    /// Membuat User baru.
    pub fn new(username: &str, role: Role) -> Self {
        Self {
            username: username.to_string(),
            role,
            perangkat: Vec::new(),
        }
    }

    /// Menambahkan perangkat ke kepemilikan user.
    pub fn tambah_perangkat(&mut self, device_id: &str) {
        if !self.perangkat.contains(&device_id.to_string()) {
            self.perangkat.push(device_id.to_string());
        }
    }

    /// Cek apakah user memiliki perangkat tertentu.
    pub fn memiliki(&self, device_id: &str) -> bool {
        self.perangkat.contains(&device_id.to_string())
    }
}

/// Pemetaan kepemilikan perangkat → user.
pub struct Kepemilikan {
    mapping: HashMap<String, String>,
}

impl Kepemilikan {
    /// Membuat Kepemilikan kosong.
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /// Menetapkan pemilik sebuah perangkat.
    pub fn tetapkan(&mut self, device_id: &str, username: &str) {
        self.mapping
            .insert(device_id.to_string(), username.to_string());
    }

    /// Mendapatkan pemilik perangkat.
    pub fn pemilik(&self, device_id: &str) -> Option<&String> {
        self.mapping.get(device_id)
    }
}

impl Default for Kepemilikan {
    fn default() -> Self {
        Self::new()
    }
}