//! Library Utama Smart Soil IoT — Modul 2: IoT Communication, Edge Processing, and User Management.
//!
//! Arsitektur mengikuti prinsip **Functional Programming (Pure Core, Imperative Shell)**:
//! - **Pure Core (100% Fungsi Murni, Immutable, Higher-Order Function, Rekursi, & Komposisi)**:
//!   - [`contracts`] — Struktur data kontrak integrasi (`RawReading`, `SoilReading`, `PlotMeta`, `QualityFlag`).
//!   - [`anomaly`] — Validasi sensor tanah gambut (sesuai RAB Modul 1), deteksi anomali edge, & kontrol interval sampling.
//!   - [`retry`] — Kebijakan retry, *exponential backoff* rekursif, & antrean *Offline Buffering* imutabel.
//!   - [`device`] — Identitas perangkat (`Device`) dan manajemen registrasi (`Registry`).
//!   - [`user`] — Model pengguna (`User`) dan pemetaan kepemilikan perangkat & lahan (`Kepemilikan`).
//!   - [`rbac`] — Role-Based Access Control (`Admin`, `Operator`, `Viewer`) & otorisasi kepemilikan.
//!   - [`pipeline`] — Komposisi fungsi *end-to-end* untuk memproses event sensor secara imutabel.
//! - **Side-Effect Shell & UI**:
//!   - [`ui`] — Visualisasi Dashboard Web Interaktif Modul 2.

pub mod anomaly;
pub mod contracts;
pub mod device;
pub mod pipeline;
pub mod rbac;
pub mod retry;
pub mod ui;
pub mod user;
