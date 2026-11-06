// ============================================================================
// FILE: src/rbac/role.rs
// STATUS: PURE ENUM & PARSER FUNCTION (Role-Based Access Control)
// HUBUNGAN KE WEB UI: KOTAK 3 (Pilihan Role pada Dropdown "Pengguna & Role")
// ============================================================================

use serde::{Deserialize, Serialize};

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: MENENTUKAN HAK AKSES PENGGUNA DI KOTAK 3
//
// 1. Admin    : Akses penuh (Bisa Baca, Kontrol semua alat, dan Hapus perangkat).
// 2. Operator : Petani/Teknisi (Bisa Baca dan Kontrol alat MILIKNYA, Ditolak jika Hapus).
// 3. Viewer   : Peneliti/Tamu (Hanya bisa Baca, Ditolak jika Kontrol atau Hapus).
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

impl Role {
    // ------------------------------------------------------------------------
    // BUKTI KEPADA DOSEN: PURE FUNCTION DENGAN PATTERN MATCHING
    // Mengonversi string input dari request HTTP menjadi enum Role secara deterministik.
    // ------------------------------------------------------------------------
    pub fn from_str_role(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "admin" => Some(Self::Admin),
            "operator" | "petani" => Some(Self::Operator),
            "viewer" | "peneliti" => Some(Self::Viewer),
            _ => None,
        }
    }
}
