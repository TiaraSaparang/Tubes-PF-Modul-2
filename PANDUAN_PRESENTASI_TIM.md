# Panduan Presentasi & Pembagian Tugas Tim (4 Orang) — Modul 2
**Penilai:** Pak Gusti Ahmad Fanshuri Alfarisy, Ph.D.

---

## Cara Menjalankan Saat Demo Besok
Buka terminal di folder `c:\Users\Tiara\Documents\PF`, lalu ketik:
```bash
# 1. Tunjukkan semua 12 Unit & Integration Test hijau (Bukti Verifikasi)
cargo test

# 2. Nyalakan Server Axum & Dashboard UI
cargo run
```
Lalu buka browser ke: **`http://localhost:3000`**

---

## Pembagian Bicara & Demo untuk 4 Orang

### 👤 ORANG 1: Tiara (Module Lead) — Arsitektur FP, Integrasi M1-M2-M3 & Dashboard UI
* **File yang dikuasai:** `src/contracts.rs`, `src/pipeline.rs`, `src/main.rs`, `src/ui.rs`
* **Apa yang disampaikan ke Pak Gusti:**
  1. **Pemisahan Pure Function vs Side-Effect (Poin Rubrik 25% + 20%):**
     *"Pak, di Modul 2 kami sudah memisahkan 100% antara **Pure Core** (`anomaly.rs`, `retry.rs`, `device`, `user`, `rbac`, `pipeline.rs`) dan **Side-Effect Shell** (`main.rs`). Di dalam Pure Core sama sekali tidak ada `Mutex`, tidak ada akses jaringan, dan tidak ada mutasi variabel (`&mut`), melainkan selalu menerima state lama dan menghasilkan state baru secara immutable."*
  2. **Kontrak Integrasi M1 ➔ M2 ➔ M3 (Poin Rubrik 20%):**
     *"Walaupun Modul 1 masih tahap RAB hardware, kami mengambil spesifikasi sensor asli dari RAB Modul 1 (`ESP32 + SIM7000E + ADS1115 + DS18B20 + DFRobot Moisture/pH/EC`) sebagai input `RawReading`. Setelah diproses di Modul 2, datanya ditransformasikan menjadi `SoilReading` beserta `PlotMeta` (`plot_id`, `name`, `latitude`, `longitude`, `area_ha`, `plant`) yang 100% sesuai dengan kontrak `ReadingSink` milik Modul 3."*
  3. **Demo di UI (`http://localhost:3000`):** Tunjukkan **Panel 4 (JSON Inspector Kontrak Modul 3)**.

---

### 👤 ORANG 2: Edge Processing, Deteksi Anomali Sensor Gambut & Sampling Interval
* **File yang dikuasai:** `src/anomaly.rs`
* **Apa yang disampaikan ke Pak Gusti:**
  1. **Spesifikasi 4 Sensor Tanah Gambut (Sesuai RAB Modul 1):**
     * **Moisture (`moisture_pct`):** DFRobot Capacitive Sensor + ADC ADS1115 (Batas fisik `0–100%`, normal gambut `30–90%`).
     * **Suhu (`temperature_c`):** DS18B20 Waterproof Probe (Batas fisik `-10..80°C`, normal gambut `18–40°C`).
     * **pH (`ph`):** Sensor pH Analog + DMS (Batas fisik `0–14`, wajar gambut `3.0–7.0`).
     * **EC (`ec_ms_cm`):** DFRobot Gravity TDS/EC (Batas fisik `0–20 mS/cm`, wajar gambut `0.05–4.0 mS/cm`).
  2. **Konsep Pemrograman Fungsional yang dipakai:**
     * **Higher-Order Function (`evaluate_with_rules`):** Menerima daftar fungsi aturan (`&[RuleFn]`) dan menyaringnya dengan `.iter().filter_map(...)` tanpa `mut Vec`.
     * **Function Composition (`validate_and_transform`):** Menggabungkan validasi identitas dan validasi angka `NaN` menggunakan `.and_then()` lalu mengubah `RawReading` menjadi `SoilReading` dengan `QualityFlag` (`Good`, `Suspect`, `Bad`).
     * **Buzzer 5V Alarm (`should_trigger_buzzer`) & Interval Control (`check_interval`):** Jika kualitas `Bad` (kabel sensor putus/dicuri), alarm Buzzer menyala. Jika pengiriman lebih cepat dari jadwal sampling, ditolak (`HTTP 429`) demi menghemat baterai 18650.
  3. **Demo di UI:** Klik 4 tombol preset di **Panel 1** (Normal, Suspect, Kabel Putus/Buzzer, dan Uji Spam Interval).

---

### 👤 ORANG 3: Konektivitas IoT (SIM7000E), Rekursi Backoff & Offline Buffering
* **File yang dikuasai:** `src/retry.rs` dan `flush_offline_buffer_pure` di `src/pipeline.rs`
* **Apa yang disampaikan ke Pak Gusti:**
  1. **Masalah di Lahan Gambut:** Sinyal seluler modem `SIM7000E` sering hilang timbul.
  2. **Konsep Pemrograman Fungsional yang dipakai:**
     * **Rekursi Murni (`hitung_backoff_ms`):** Menghitung jeda waktu tunggu *exponential backoff* (`500ms -> 1000ms -> 2000ms`) menggunakan fungsi rekursif murni.
     * **Immutability pada Offline Buffer (`enqueue_offline` & `take_reconnect_batch`):** Saat sinyal mati, data tidak hilang tetapi dimasukkan ke antrean menggunakan `.iter().cloned().chain(...)` yang menghasilkan `Vec` baru tanpa mengubah antrean asli.
     * **Higher-Order Fold (`flush_offline_buffer_pure`):** Saat koneksi kembali *Online*, seluruh isi antrean offline dikirim otomatis ke Modul 3 menggunakan `.into_iter().fold(...)`.
  3. **Demo di UI:** Di **Panel 2**, klik **"Simulasi Sinyal Putus (Set OFFLINE)"**, lalu kirim 2–3 data sensor di Panel 1 (terlihat masuk ke tabel Offline Buffer). Lalu klik **"Reconnect & Upload Buffer ke M3 (Set ONLINE)"** — semua data otomatis pindah ke tabel Modul 3!

---

### 👤 ORANG 4: Device Provisioning, Pemetaan Pemilik (`PlotMeta`) & RBAC
* **File yang dikuasai:** `src/device/`, `src/user/`, `src/rbac/`
* **Apa yang disampaikan ke Pak Gusti:**
  1. **Identitas & Kepemilikan Alat:** Setiap alat terdaftar di `Registry` dan dipetakan di `Kepemilikan` ke pemilik (`username`) serta petak lahan (`PlotMeta`).
  2. **Konsep Pemrograman Fungsional yang dipakai:**
     * **Immutable Updates (`registrasi_murni`, `ubah_interval_murni`, `tetapkan_lengkap_murni`):** Setiap perubahan data perangkat/user selalu menghasilkan struct baru tanpa mutasi langsung.
     * **Pure Pattern Matching & Komposisi Otorisasi (`validasi_akses` & `otorisasi_aksi_perangkat`):**
       * `Admin`: Boleh Baca, Kontrol (ubah interval), dan Hapus semua perangkat.
       * `Operator` (Petani): Boleh Baca dan Kontrol **hanya perangkat miliknya sendiri**, tidak boleh Hapus.
       * `Viewer` (Peneliti): Hanya boleh Baca.
  3. **Demo di UI:** Di **Panel 3**, coba pilih `petani_budi (Operator)` lalu ubah interval `ESP32-GAMBUT-01` (Sukses karena miliknya), lalu coba ubah `ESP32-GAMBUT-02` milik `petani_siti` (Ditolak karena bukan miliknya!), lalu coba `peneliti_rina (Viewer)` untuk Kontrol (Ditolak RBAC!).
