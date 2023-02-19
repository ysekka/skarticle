#[derive(Clone, Debug, serde::Deserialize)]
pub struct MailSettings {
    pub email_username: String,
    pub email_password: String,
    pub email_smtp: String,
}