# Planning Module — Modul 2: IoT Communication, Edge Processing, and User Management

Dokumen ini merangkum seluruh fungsi murni (*Pure Functions*) dan fungsi berefek samping (*Side-Effect Functions*) pada Modul 2.

## 1. Tabel Fungsi Murni (Pure Core)

| Modul | Fungsi | Signature | Konsep FP & Deskripsi |
|-------|--------|-----------|-----------------------|
| `anomaly` | `check_interval` | `fn(u64, u64, u64) -> bool` | **Pure Function**: Mengecek apakah jadwal sampling perangkat sudah tercapai. |
| `anomaly` | `classify_moisture` / `temperature` / `ph` / `ec` | `fn(f32) -> SensorStatus` | **Pure Function**: Mengklasifikasikan 4 parameter sensor tanah gambut sesuai spesifikasi RAB Modul 1. |
| `anomaly` | `evaluate_with_rules` | `fn(&RawReading, &[RuleFn]) -> Vec<String>` | **Higher-Order Function**: Menerima slice fungsi aturan (`RuleFn`) dan mengevaluasinya dengan `filter_map`. |
| `anomaly` | `validate_and_transform` | `fn(&RawReading, &PlotMeta) -> Result<SoilReading, String>` | **Function Composition (`.and_then`)**: Memvalidasi data mentah lalu mentransformasikannya menjadi `SoilReading` untuk Modul 3. |
| `retry` | `should_retry` | `fn(Option<u16>) -> bool` | **Pure Pattern Matching**: Menentukan apakah kode HTTP perlu dicoba ulang. |
| `retry` | `hitung_backoff_ms` | `fn(u32, u64) -> u64` | **Pure Recursion**: Menghitung jeda waktu *exponential backoff* secara rekursif. |
| `retry` | `enqueue_offline` | `fn(&[BufferedItem], RawReading, &str) -> Vec<BufferedItem>` | **Immutability**: Menambahkan pembacaan ke antrean offline tanpa mengubah slice asli. |
| `retry` | `take_reconnect_batch` | `fn(&[BufferedItem], usize) -> (Vec<BufferedItem>, Vec<BufferedItem>)` | **Immutability**: Memisahkan batch antrean offline yang siap di-upload saat koneksi pulih. |
| `device` | `Registry::registrasi_murni` | `fn(&self, Device) -> Result<Registry, String>` | **Immutability**: Menghasilkan `Registry` baru dengan perangkat tambahan tanpa mutasi `&mut`. |
| `device` | `Registry::ubah_interval_murni` | `fn(&self, &str, u64) -> Result<Registry, String>` | **Immutability**: Menghasilkan `Registry` baru dengan interval sampling perangkat yang diperbarui. |
| `user` | `Kepemilikan::tetapkan_lengkap_murni` | `fn(&self, &str, &str, PlotMeta) -> Kepemilikan` | **Immutability**: Memetakan `device_id` ke `username` dan `PlotMeta` (kontrak Modul 3). |
| `rbac` | `validasi_akses` | `fn(&Role, &Aksi) -> bool` | **Pure Pattern Matching**: Matriks RBAC (`Admin`, `Operator`, `Viewer` terhadap `Baca`, `Kontrol`, `Hapus`). |
| `rbac` | `otorisasi_aksi_perangkat` | `fn(&str, &Role, &Aksi, &str, &Kepemilikan) -> Result<String, String>` | **Pure Composition**: Menggabungkan validasi RBAC dengan kepemilikan perangkat. |
| `pipeline` | `process_sensor_event_pure` | `fn(&GatewaySnapshot, RawReading) -> (GatewaySnapshot, PipelineOutcome)` | **Pure State Transition**: Menjalankan seluruh alur Modul 2 tanpa efek samping. |
| `pipeline` | `flush_offline_buffer_pure` | `fn(&GatewaySnapshot) -> (GatewaySnapshot, Vec<PipelineOutcome>)` | **Higher-Order Fold**: Mengirimkan seluruh isi *Offline Buffer* menggunakan `.fold()` saat kembali *Online*. |

## 2. Tabel Fungsi Berefek Samping (Side-Effect Shell)

| Modul | Fungsi | Signature | Efek Samping |
|-------|--------|-----------|--------------|
| `retry` | `send_with_retry` | `async fn(&str, &str, &RawReading, u32) -> Result<(), String>` | Pengiriman jaringan HTTP POST asinkron menggunakan `reqwest` dan timer `tokio::time::sleep`. |
| `main` | `sensor_handler`, `network_mode_handler`, `rbac_action_handler`, `register_device_handler` | Handler HTTP Axum | Menerima request jaringan HTTP dan memperbarui state server. |

## 3. Matriks Hak Akses (RBAC)

| Role | Baca Data | Kontrol (Ubah Interval Sampling) | Hapus Perangkat |
|------|-----------|----------------------------------|-----------------|
| **Admin** | Ya (Semua Alat) | Ya (Semua Alat) | Ya (Semua Alat) |
| **Operator (Petani)** | Ya (Semua Alat) | Ya (Hanya Alat Miliknya Sendiri) | Ditolak |
| **Viewer (Peneliti)** | Ya (Semua Alat) | Ditolak | Ditolak |
