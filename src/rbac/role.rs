#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// Admin — akses penuh.
    Admin,
    /// Operator — bisa kontrol, tidak bisa hapus.
    Operator,
    /// Viewer — hanya bisa lihat.
    Viewer,
}