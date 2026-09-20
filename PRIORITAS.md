# Prioritas Modul — Device-User Mapping

## Latar Belakang

Modul **Device-User Mapping** menangani tiga hal utama:
1. Registrasi ID perangkat
2. Pemetaan kepemilikan perangkat ke akun pengguna
3. Validasi hak akses (Role-Based Access Control)

Ketiga hal ini adalah **fondasi** yang dibutuhkan oleh modul-modul lain
dalam sistem Smart Soil IoT.

## Urutan Prioritas

| Prioritas | Modul | Alasan |
|-----------|-------|--------|
| 1 | `device` | Registrasi ID perangkat adalah **fondasi utama**. Tanpa identitas perangkat yang jelas, sistem tidak bisa membedakan satu alat dengan yang lain. Semua modul lain bergantung pada data perangkat ini. |
| 2 | `user` | Pemetaan kepemilikan dibutuhkan setelah perangkat terdaftar. Fitur ini memungkinkan sistem tahu **siapa pemilik perangkat**, sehingga akses bisa dibatasi sesuai pemilik. |
| 3 | `rbac` | Validasi hak akses (RBAC) baru bisa berjalan **setelah** data device & user tersedia. RBAC bergantung pada dua modul sebelumnya. |

## Justifikasi Pemilihan Prioritas

Urutan prioritas dipilih berdasarkan **ketergantungan antar-modul**:
