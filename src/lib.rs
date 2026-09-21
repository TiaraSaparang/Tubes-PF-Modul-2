//! Library utama untuk sistem manajemen perangkat.
//!
//! Modul:
//! - [`device`] — Registrasi perangkat
//! - [`user`] — User & kepemilikan
//! - [`rbac`] — Validasi hak akses

pub mod device;
pub mod rbac;
pub mod user;
pub mod retry;
pub mod anomaly;