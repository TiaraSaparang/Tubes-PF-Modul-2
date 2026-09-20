# Planning Module — Device-User Mapping

Modul ini menangani registrasi perangkat, pemetaan kepemilikan perangkat 
ke pengguna, dan validasi hak akses (RBAC).

## Tabel Fungsi

| Fungsi | Signature | Deskripsi |
|--------|-----------|-----------|
| `Device::new` | `fn new(id: &str, nama: &str, tipe: &str) -> Self` | Membuat instance `Device` baru dengan id, nama, dan tipe. |
| `Registry::new` | `fn new() -> Self` | Membuat `Registry` kosong untuk menyimpan daftar perangkat. |
| `Registry::registrasi` | `fn registrasi(&mut self, device: Device) -> Result<(), String>` | Mendaftarkan perangkat baru. Mengembalikan error jika ID sudah terdaftar. |
| `Registry::cari` | `fn cari(&self, id: &str) -> Option<&Device>` | Mencari perangkat berdasarkan ID. Mengembalikan `None` jika tidak ditemukan. |
| `Registry::hapus` | `fn hapus(&mut self, id: &str) -> bool` | Menghapus perangkat dari registry. Mengembalikan `true` jika berhasil. |
| `Registry::jumlah` | `fn jumlah(&self) -> usize` | Mengembalikan jumlah perangkat yang terdaftar. |
| `User::new` | `fn new(username: &str, role: Role) -> Self` | Membuat `User` baru dengan username dan role. |
| `User::tambah_perangkat` | `fn tambah_perangkat(&mut self, device_id: &str)` | Menambahkan perangkat ke daftar kepemilikan user. |
| `User::memiliki` | `fn memiliki(&self, device_id: &str) -> bool` | Cek apakah user memiliki perangkat tertentu. |
| `Kepemilikan::new` | `fn new() -> Self` | Membuat pemetaan kepemilikan kosong. |
| `Kepemilikan::tetapkan` | `fn tetapkan(&mut self, device_id: &str, username: &str)` | Menetapkan pemilik sebuah perangkat. |
| `Kepemilikan::pemilik` | `fn pemilik(&self, device_id: &str) -> Option<&String>` | Mendapatkan pemilik sebuah perangkat. |
| `validasi_akses` | `fn validasi_akses(role: &Role, aksi: &Aksi) -> bool` | Memvalidasi apakah role tertentu boleh melakukan aksi tertentu (RBAC). |

## Aturan Akses (RBAC)

| Role | Baca | Kontrol | Hapus |
|------|------|---------|-------|
| Admin | ✅ | ✅ | ✅ |
| Operator | ✅ | ✅ | ❌ |
| Viewer | ✅ | ❌ | ❌ |