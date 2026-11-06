// ============================================================================
// PANDUAN PETA KODINGAN TERHADAP WEB UI (DASHBOARD http://localhost:3000)
// ARSITEKTUR: PURE CORE, IMPERATIVE SHELL (PEMROGRAMAN FUNGSIONAL RUST)
// ============================================================================
//
// 1. KOTAK 1 (Input Data Sensor Edge & Kontrol Interval):
//    - src/contracts.rs -> Struct RawReading
//    - src/anomaly.rs   -> check_interval (hemat baterai 18650), validate_and_transform,
//                          rule_moisture, rule_temperature, rule_ph, rule_ec, should_trigger_buzzer
//
// 2. KOTAK 2 (Simulasi Koneksi & Offline Buffering):
//    - src/retry.rs     -> hitung_backoff_ms (rekursi murni), enqueue_offline (immutability),
//                          take_reconnect_batch (irisan buffer murni)
//
// 3. KOTAK 3 (Registrasi Perangkat, Pemetaan Pemilik & RBAC):
//    - src/device/model.rs      -> Device::with_sampling_interval (immutable struct)
//    - src/device/registrasi.rs -> Registry::registrasi_murni, ubah_interval_murni, hapus_murni
//    - src/user/model.rs        -> User::tambah_perangkat_murni, Kepemilikan::tetapkan_lengkap_murni
//    - src/rbac/role.rs         -> Role::Admin, Operator, Viewer
//    - src/rbac/validasi.rs     -> validasi_akses (matriks RBAC), otorisasi_aksi_perangkat (komposisi)
//
// 4. KOTAK 4 (Output Kontrak ke Modul 3):
//    - src/contracts.rs -> SoilReading, PlotMeta, QualityFlag (Good, Suspect, Bad)
//    - Endpoint GET /api/m3/readings -> Kontrak resmi untuk ReadingSink Modul 3
//
// 5. JANTUNG PIPA PEMROSESAN (PURE CORE ENGINE):
//    - src/pipeline.rs  -> process_sensor_event_pure (transisi state murni tanpa side-effect),
//                          flush_offline_buffer_pure (.fold() murni saat upload-on-reconnect)
//
// 6. SERVER HTTP & EFEK SAMPING (SIDE-EFFECT SHELL):
//    - src/main.rs      -> Axum Web Server, Mutex state, middleware authenticate
//    - src/ui.rs        -> Tampilan visual dashboard HTML/CSS
// ============================================================================

pub mod anomaly;
pub mod contracts;
pub mod device;
pub mod pipeline;
pub mod rbac;
pub mod retry;
pub mod ui;
pub mod user;
