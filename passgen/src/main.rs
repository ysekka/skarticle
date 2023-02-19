use actix_rt as ar;

use lettre::Transport;
use migration::sea_orm as so;

use migration::MigratorTrait;
use sha2::Digest;
use so::EntityTrait;

#[ar::main]
async fn main() {
    use common::settings::r#struct as stcSettings;

    let config_path = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle");

    let settings_path = config_path.clone()
    .join("config.ron");

    let password_path = config_path.clone()
    .join("password");

    let settings = stcSettings::Settings::from_path(&settings_path);

    let database_connection = so::Database::connect(settings.database_url.as_str()).await
    .expect("Unable to establish database connection.");

    migration::Migrator::up(&database_connection, None).await
    .expect("Unable to migrate up.");

    let credentials = lettre::transport::smtp::authentication::Credentials::new(
        settings.email.email_username.to_owned(),
        settings.email.email_password.to_owned()
    );

    let mailer = lettre::SmtpTransport::starttls_relay(&settings.email.email_smtp)
    .unwrap()
    .credentials(credentials)
    .build();

    loop {
        if !password_path.exists() {
            std::fs::File::create(password_path.clone())
            .expect("Could not create password file.");
        }

        let random_value = (rand::random::<f32>() * 100000000.0).ceil() as usize;
        let mut hasher = sha2::Sha256::new();
        hasher.update(random_value.to_string());
        let hashed_value = hasher.finalize();

        std::fs::write(password_path.clone(), hashed_value)
        .expect("Could not write password over.");

        let users_query = entity::users_table::Entity::find()
        .all(&database_connection).await;

        if let Ok(users) = users_query {
            for user in users.iter() {
                let email = lettre::Message::builder()
                .from(lettre::message::Mailbox::new(Some("SKARTICLE".to_owned()), settings.email.email_username.parse().unwrap()))
                .to(lettre::message::Mailbox::new(None, user.user_email.parse().unwrap()))
                .subject("Yeni Şifre - SKARTICAL")
                .body(format!("Yeni Şifreniz: {}", hashed_value.iter().map(|x| format!("{x:02x}")).collect::<String>()))
                .expect("Could not create email message.");

                mailer.send(&email)
                .expect("Could not send message.");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(60 * 60 * 24));
    }
}
