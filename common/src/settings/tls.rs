#[derive(Clone, Debug, serde::Deserialize)]
pub struct TlsSettings {
    pub tls_key: Box<std::path::Path>,
    pub tls_certificate: Box<std::path::Path>,
}