use migration::sea_orm as so;
use env_logger as el;
use actix_web as aw;

use common::settings::r#struct as stcSettings;
use crate::state::r#struct as stcState;

use migration::MigratorTrait;

pub mod routes;
pub mod state;

#[aw::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::error_response::error_response;
    use crate::routes::configure;

    let settings_path = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle")
    .join("config.ron");

    let settings = stcSettings::Settings::from_path(&settings_path);

    if settings.logging {
        el::init_from_env(el::Env::default().default_filter_or(match settings.mode.as_str() {
            "DEVELOPMENT" => "DEBUG",
            "PRODUCTION" => "INFO",
            _ => panic!("Invalid application mode.")
        }))
    }

    let database_connection = so::Database::connect(settings.database_url.as_str()).await
    .expect("Error occured during establishing database connection.");

    migration::Migrator::up(&database_connection, None).await
    .expect("Error occured during migrating up.");

    let app_state = stcState::State {
        default_thumbnail: settings.default_thumbnail,
        public_directory: settings.public_directory,
        database_connection
    };

    let mut server = aw::HttpServer::new(move || {
        aw::App::new()
        .app_data(aw::web::Data::new(app_state.clone()))
        .wrap(aw::middleware::ErrorHandlers::new().default_handler(error_response))
        .wrap(aw::middleware::Logger::default())
        .configure(configure)
    });

    for bind_settings in settings.server.iter() {
        let mut ssl_builder = openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
        .expect("Error occured during building of ssl acceptor.");

        ssl_builder.set_private_key_file(&settings.tls.tls_key, openssl::ssl::SslFiletype::PEM)
        .expect("Error occured during setting of key file.");

        ssl_builder.set_certificate_chain_file(&settings.tls.tls_certificate)
        .expect("Error occured during setting of certificate file.");

        server = server.bind_openssl((
            bind_settings.server_host,
            bind_settings.server_port
        ), ssl_builder)
        .expect("Error occured during binding.");
    }

    server.run().await
}
