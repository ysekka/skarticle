#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerSettings {
    pub server_host: std::net::IpAddr,
    pub server_port: u16,
}