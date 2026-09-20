//! Modul model untuk perangkat.

/// Representasi sebuah perangkat.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub id: String,
    pub nama: String,
    pub tipe: String,
}

impl Device {
    /// Membuat Device baru.
    pub fn new(id: &str, nama: &str, tipe: &str) -> Self {
        Self {
            id: id.to_string(),
            nama: nama.to_string(),
            tipe: tipe.to_string(),
        }
    }
}