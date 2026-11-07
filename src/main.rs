// ============================================================================
// FILE: src/main.rs
// STATUS: SIDE-EFFECT SHELL (Axum Web Server, I/O Jaringan, Mutex State)
// HUBUNGAN KE WEB UI: PINTU GERBANG UTAMA YANG MENANGANI SELURUH TOMBOL & TAMPILAN DASHBOARD
// ============================================================================
//
// PENJELASAN KEPADA DOSEN TENTANG ARSITEKTUR KITA:
// Sistem ini menerapkan arsitektur "Pure Core, Imperative Shell":
// - PURE CORE: Seluruh aturan bisnis, validasi sensor tanah, algoritma backoff rekursif,
//   dan otorisasi RBAC dibuat 100% murni tanpa efek samping di modul-modul lain.
// - SIDE-EFFECT SHELL: File main.rs ini bertugas menangani efek samping I/O,
//   menerima request HTTP dari browser/ESP32, dan mengunci memori Mutex state.
// ============================================================================

use axum::{
    extract::{Json, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use tubes_pf_modul_2::{
    contracts::{M3IngestEnvelope, PlotMeta, RawReading},
    device::Device,
    pipeline::{
        flush_offline_buffer_pure, process_sensor_event_pure, GatewaySnapshot, PipelineOutcome,
    },
    rbac::{otorisasi_aksi_perangkat, verify_api_key, Aksi, Role},
    retry::BufferedItem,
    ui::DASHBOARD_HTML,
    user::{verifikasi_login, User},
};

#[derive(Clone)]
pub struct AppState {
    pub api_key: String,
    pub snapshot: Arc<Mutex<GatewaySnapshot>>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<User>,
}

#[derive(Serialize)]
pub struct DeviceView {
    pub id: String,
    pub nama: String,
    pub tipe: String,
    pub sampling_interval_sec: u64,
    pub owner: Option<String>,
    pub plot: Option<PlotMeta>,
}

#[derive(Serialize)]
pub struct StateView {
    pub network_online: bool,
    pub devices: Vec<DeviceView>,
    pub offline_buffer: Vec<BufferedItem>,
    pub m3_sink_records: Vec<M3IngestEnvelope>,
    pub users: Vec<User>,
}

#[derive(Deserialize)]
pub struct NetworkModePayload {
    pub online: bool,
}

#[derive(Deserialize)]
pub struct RbacActionPayload {
    pub username: String,
    pub role: String,
    pub aksi: String,
    pub device_id: String,
    pub new_interval_sec: Option<u64>,
}

#[derive(Deserialize)]
pub struct RegisterDevicePayload {
    pub device_id: String,
    pub nama: String,
    pub tipe: String,
    pub owner_username: String,
    pub plot_meta: PlotMeta,
}

// ----------------------------------------------------------------------------
// SIDE-EFFECT SHELL: MIDDLEWARE KEAMANAN HTTP (X-API-Key)
// Membaca header request HTTP dan memverifikasinya menggunakan fungsi murni `verify_api_key`.
// ----------------------------------------------------------------------------
async fn authenticate(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let provided_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok());

    match verify_api_key(provided_key, &state.api_key) {
        Ok(()) => next.run(request).await,
        Err(msg) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                success: false,
                message: msg,
            }),
        )
            .into_response(),
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: MENYAJIKAN HALAMAN WEB DASHBOARD (GET http://localhost:3000/)
// Mengembalikan file HTML, CSS biru lembut, dan JavaScript ke browser pengguna.
// ----------------------------------------------------------------------------
async fn dashboard_handler() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: AUTENTIKASI BACKEND (POST /api/auth/login)
// Memvalidasi kredensial login menggunakan fungsi murni `verifikasi_login`.
// ----------------------------------------------------------------------------
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let guard = state.snapshot.lock().unwrap();
    match verifikasi_login(&guard.users, &payload.username, &payload.password) {
        Ok(user) => (
            StatusCode::OK,
            Json(LoginResponse {
                success: true,
                message: format!(
                    "Login berhasil! Selamat datang, {} (Peran: {:?})",
                    user.username, user.role
                ),
                user: Some(user.clone()),
            }),
        ),
        Err(err_msg) => (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                message: err_msg,
                user: None,
            }),
        ),
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: LOGOUT BACKEND (POST /api/auth/logout)
// ----------------------------------------------------------------------------
async fn logout_handler() -> impl IntoResponse {
    Json(ApiResponse {
        success: true,
        message: "Berhasil logout dari sistem Gateway".to_string(),
    })
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: MEMPERBARUI STATUS BAR & TABEL-TABEL UI SECARA OTOMATIS (GET /api/state)
// Browser memanggil rute ini secara berkala untuk me-refresh data di layar secara realtime.
// ----------------------------------------------------------------------------
async fn get_state_handler(State(state): State<AppState>) -> impl IntoResponse {
    let snap = state.snapshot.lock().unwrap().clone();

    let devices: Vec<DeviceView> = snap
        .registry
        .daftar_semua()
        .into_iter()
        .map(|d| {
            let owner = snap.kepemilikan.pemilik(&d.id).cloned();
            let plot = snap.kepemilikan.plot_of(&d.id).cloned();
            DeviceView {
                id: d.id,
                nama: d.nama,
                tipe: d.tipe,
                sampling_interval_sec: d.sampling_interval_sec,
                owner,
                plot,
            }
        })
        .collect();

    Json(StateView {
        network_online: snap.network_online,
        devices,
        offline_buffer: snap.offline_buffer,
        m3_sink_records: snap.m3_sink_records,
        users: snap.users,
    })
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE KONTRAK MODUL 3: ENDPOINT INTEGRASI PEMBACAAN DATA (GET /api/m3/readings)
// Tim Modul 3 (ReadingSink) mengambil seluruh data sensor yang telah divalidasi lewat sini.
// ----------------------------------------------------------------------------
async fn get_m3_readings_handler(State(state): State<AppState>) -> impl IntoResponse {
    let snap = state.snapshot.lock().unwrap().clone();
    Json(snap.m3_sink_records)
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Kirim Data Sensor" DI KOTAK 1 DI-KLIK
// (Rute: POST /api/sensor/data)
//
// ALUR KERJA:
// 1. Menerima payload JSON RawReading dari browser.
// 2. Meneruskan ke fungsi murni `process_sensor_event_pure` di pipeline.rs.
// 3. Mengembalikan status HTTP (200 = Sukses Modul 3, 202 = Masuk Buffer, 400/429 = Ditolak).
// ----------------------------------------------------------------------------
async fn sensor_handler(
    State(state): State<AppState>,
    Json(payload): Json<RawReading>,
) -> impl IntoResponse {
    let mut guard = state.snapshot.lock().unwrap();
    let (next_snapshot, outcome) = process_sensor_event_pure(&guard, payload);
    *guard = next_snapshot;

    match outcome {
        PipelineOutcome::ForwardedToM3 { message, .. } => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message,
            }),
        ),
        PipelineOutcome::BufferedOffline { message, .. } => (
            StatusCode::ACCEPTED,
            Json(ApiResponse {
                success: true,
                message,
            }),
        ),
        PipelineOutcome::Rejected { code, reason } => {
            let status = StatusCode::from_u16(code).unwrap_or(StatusCode::BAD_REQUEST);
            (
                status,
                Json(ApiResponse {
                    success: false,
                    message: reason,
                }),
            )
        }
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL MODE KONEKSI DI KOTAK 2 DI-KLIK
// (Rute: POST /api/network/mode)
//
// 1. Tombol "Matikan Koneksi": Mengubah status ke Offline.
// 2. Tombol "Sambungkan Kembali & Kirim Buffer": Mengubah status ke Online dan
//    secara otomatis memanggil `flush_offline_buffer_pure` untuk menguras antrean buffer.
// ----------------------------------------------------------------------------
async fn network_mode_handler(
    State(state): State<AppState>,
    Json(payload): Json<NetworkModePayload>,
) -> impl IntoResponse {
    let mut guard = state.snapshot.lock().unwrap();

    if payload.online {
        let buffered_len = guard.offline_buffer.len();
        let (next_snap, outcomes) = flush_offline_buffer_pure(&guard);
        *guard = next_snap;
        let success_count = outcomes
            .iter()
            .filter(|o| matches!(o, PipelineOutcome::ForwardedToM3 { .. }))
            .count();

        (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: format!(
                    "Koneksi SIM7000E kembali ONLINE! Otomatis meng-upload {} dari {} data di Offline Buffer ke ReadingSink Modul 3.",
                    success_count, buffered_len
                ),
            }),
        )
    } else {
        *guard = GatewaySnapshot {
            network_online: false,
            ..guard.clone()
        };
        (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Simulasi sinyal SIM7000E terputus (Mode OFFLINE aktif). Semua kiriman sensor berikutnya akan disimpan ke Offline Buffer.".to_string(),
            }),
        )
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Jalankan Aksi" DI KOTAK 3 DI-KLIK
// (Rute: POST /api/rbac/action)
//
// ALUR KERJA:
// 1. Memeriksa izin RBAC & Kepemilikan melalui fungsi murni `otorisasi_aksi_perangkat`.
// 2. Jika diizinkan:
//    - Aksi 'Baca'    -> Menampilkan notifikasi sukses.
//    - Aksi 'Kontrol' -> Memanggil fungsi murni `ubah_interval_murni`.
//    - Aksi 'Hapus'   -> Memanggil fungsi murni `hapus_murni`.
// ----------------------------------------------------------------------------
async fn rbac_action_handler(
    State(state): State<AppState>,
    Json(payload): Json<RbacActionPayload>,
) -> impl IntoResponse {
    let role = match Role::from_str_role(&payload.role) {
        Some(r) => r,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    message: "Role tidak dikenali (gunakan Admin, Operator, atau Viewer)"
                        .to_string(),
                }),
            );
        }
    };

    let aksi = match Aksi::from_str_aksi(&payload.aksi) {
        Some(a) => a,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    message: "Aksi tidak dikenali (gunakan Baca, Kontrol, atau Hapus)".to_string(),
                }),
            );
        }
    };

    let mut guard = state.snapshot.lock().unwrap();

    // Panggil fungsi murni otorisasi RBAC + Kepemilikan
    if let Err(err_msg) = otorisasi_aksi_perangkat(
        &payload.username,
        &role,
        &aksi,
        &payload.device_id,
        &guard.kepemilikan,
    ) {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                success: false,
                message: err_msg,
            }),
        );
    }

    // Jika otorisasi sukses, jalankan aksi secara immutable
    match aksi {
        Aksi::Baca => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: format!(
                    "RBAC Sukses: '{}' ({:?}) diizinkan membaca data perangkat '{}'.",
                    payload.username, role, payload.device_id
                ),
            }),
        ),
        Aksi::Kontrol => {
            let new_interval = payload.new_interval_sec.unwrap_or(60);
            match guard
                .registry
                .ubah_interval_murni(&payload.device_id, new_interval)
            {
                Ok(next_reg) => {
                    *guard = GatewaySnapshot {
                        registry: next_reg,
                        ..guard.clone()
                    };
                    (
                        StatusCode::OK,
                        Json(ApiResponse {
                            success: true,
                            message: format!(
                                "RBAC & Ownership Sukses: '{}' ({:?}) berhasil mengubah interval sampling '{}' menjadi {} detik.",
                                payload.username, role, payload.device_id, new_interval
                            ),
                        }),
                    )
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        message: e,
                    }),
                ),
            }
        }
        Aksi::Hapus => match guard.registry.hapus_murni(&payload.device_id) {
            Ok(next_reg) => {
                *guard = GatewaySnapshot {
                    registry: next_reg,
                    ..guard.clone()
                };
                (
                    StatusCode::OK,
                    Json(ApiResponse {
                        success: true,
                        message: format!(
                            "RBAC Sukses: '{}' ({:?}) menghapus perangkat '{}' dari Registry.",
                            payload.username, role, payload.device_id
                        ),
                    }),
                )
            }
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    message: e,
                }),
            ),
        },
    }
}

// ----------------------------------------------------------------------------
// HUBUNGAN KE UI: DIPANGGIL SAAT TOMBOL "Simpan Perangkat" DI KOTAK 3 DI-KLIK
// (Rute: POST /api/devices/register)
//
// Mendaftarkan perangkat baru, pemiliknya, dan petak lahannya (PlotMeta)
// menggunakan fungsi murni `registrasi_murni` dan `tetapkan_lengkap_murni`.
// ----------------------------------------------------------------------------
async fn register_device_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDevicePayload>,
) -> impl IntoResponse {
    let mut guard = state.snapshot.lock().unwrap();
    let new_dev = Device::new(&payload.device_id, &payload.nama, &payload.tipe);

    match guard.registry.registrasi_murni(new_dev) {
        Ok(next_reg) => {
            let next_kep = guard.kepemilikan.tetapkan_lengkap_murni(
                &payload.device_id,
                &payload.owner_username,
                payload.plot_meta.clone(),
            );
            *guard = GatewaySnapshot {
                registry: next_reg,
                kepemilikan: next_kep,
                ..guard.clone()
            };
            (
                StatusCode::CREATED,
                Json(ApiResponse {
                    success: true,
                    message: format!(
                        "Perangkat '{}' berhasil didaftarkan untuk pemilik '{}' pada petak '{}' ({})",
                        payload.device_id,
                        payload.owner_username,
                        payload.plot_meta.plot_id,
                        payload.plot_meta.plant
                    ),
                }),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: e,
            }),
        ),
    }
}

// ----------------------------------------------------------------------------
// ROUTER SETUP: MENGHUBUNGKAN SELURUH URL ENDPOINT KE HANDLER MASING-MASING
// ----------------------------------------------------------------------------
pub fn build_app(state: AppState) -> Router {
    let protected_api = Router::new()
        .route("/api/sensor/data", post(sensor_handler))
        .route("/api/network/mode", post(network_mode_handler))
        .route("/api/rbac/action", post(rbac_action_handler))
        .route("/api/devices/register", post(register_device_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), authenticate));

    Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/state", get(get_state_handler))
        .route("/api/m3/readings", get(get_m3_readings_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/logout", post(logout_handler))
        .merge(protected_api)
        .with_state(state)
}

// ----------------------------------------------------------------------------
// ENTRY POINT (SIDE-EFFECT SHELL): MEMULAI WEB SERVER HTTP
// ----------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    let state = AppState {
        api_key: "TubesPF-demo-key".to_string(),
        snapshot: Arc::new(Mutex::new(GatewaySnapshot::initial_seed())),
    };

    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("==========================================================");
    println!("Smart Soil IoT - Modul 2 (Gateway, Edge & RBAC) Aktif");
    println!("Dashboard UI    : http://localhost:3000");
    println!("Endpoint Login  : POST http://localhost:3000/api/auth/login");
    println!("Endpoint Sensor : POST http://localhost:3000/api/sensor/data");
    println!("Endpoint Sink M3: GET  http://localhost:3000/api/m3/readings");
    println!("Header X-API-Key: TubesPF-demo-key");
    println!("==========================================================");

    axum::serve(listener, app).await.unwrap();
}
