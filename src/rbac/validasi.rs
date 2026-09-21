//! Modul validasi hak akses (RBAC).

use super::role::Role;

/// Aksi yang bisa dilakukan terhadap perangkat.
#[derive(Debug, Clone, PartialEq)]
pub enum Aksi {
    /// Melihat data perangkat.
    Baca,
    /// Mengontrol perangkat.
    Kontrol,
    /// Menghapus perangkat.
    Hapus,
}

/// Memvalidasi apakah role boleh melakukan aksi.
pub fn validasi_akses(role: &Role, aksi: &Aksi) -> bool {
    match (role, aksi) {
        (Role::Admin, _) => true,
        (Role::Operator, Aksi::Baca) => true,
        (Role::Operator, Aksi::Kontrol) => true,
        (Role::Viewer, Aksi::Baca) => true,
        _ => false,
    }
}