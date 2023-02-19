use super::server::ServerSettings;
use super::smtp::MailSettings;
use super::tls::TlsSettings;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Settings {
    pub server: Vec<ServerSettings>,
    pub default_thumbnail: String,
    pub public_directory: String,
    pub email: MailSettings,
    pub database_url: Url,
    pub tls: TlsSettings,
    pub logging: bool,
    pub mode: String,
}