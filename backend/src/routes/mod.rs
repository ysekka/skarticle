use actix_web_httpauth as awh;
use actix_files as af;
use actix_web as aw;

use common::settings::r#struct as stcSettings;

pub mod authentication;
pub mod error_response;
pub mod remove_article;
pub mod create_article;
pub mod update_article;
pub mod remove_user;
pub mod create_user;
pub mod update_user;
pub mod get_article;
pub mod get_user;

pub mod handle_files;
pub mod run_query;

pub fn configure(configurations: &mut aw::web::ServiceConfig) {
    use crate::routes::authentication::{
        administration_authentication,
        normal_authentication
    };

    use crate::routes::{
        remove_user::remove_user,
        create_user::create_user,
        update_user::update_user,
        run_query::run_query,
        create_article::create_article,
        remove_article::remove_article,
        update_article::update_article,
        handle_files::{
            remove_file,
            upload_file
        },
        get_user::{
            get_self,
            get_user,
        },
        get_article::{
            get_article,
            get_article_all,
        }
    };

    let settings_path = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle")
    .join("config.ron");

    let settings = stcSettings::Settings::from_path(&settings_path);

    configurations
    .service(
        af::Files::new("/public", &settings.public_directory)
        .use_last_modified(true)
        .show_files_listing()
    )
    .service(
        aw::web::scope("/api")
        .app_data(aw::web::PayloadConfig::new(1073741824))
        .service(
            aw::web::scope("/article")
            .route("/get", aw::web::get().to(get_article))
        )
        .service(
            aw::web::scope("/private")
            .wrap(awh::middleware::HttpAuthentication::bearer(normal_authentication))
            .service(upload_file)
            .service(remove_file)
            .service(
                aw::web::scope("/article")
                .route("/get", aw::web::get().to(get_article_all))
                .route("/update", aw::web::put().to(update_article))
                .route("/create", aw::web::post().to(create_article))
                .route("/remove", aw::web::delete().to(remove_article))
            )
            .service(
                aw::web::scope("/user")
                .route("/self", aw::web::get().to(get_self))
                .route("/update", aw::web::put().to(update_user))
            )
            .service(
                aw::web::scope("/administration")
                .wrap(awh::middleware::HttpAuthentication::bearer(administration_authentication))
                .route("/run", aw::web::method(aw::http::Method::from_bytes(b"EXEC").unwrap()).to(run_query))
                .service(
                    aw::web::scope("/user")
                    .route("/get", aw::web::get().to(get_user))
                    .route("/create", aw::web::post().to(create_user))
                    .route("/remove", aw::web::delete().to(remove_user))
                )
            )
        )
    );
}