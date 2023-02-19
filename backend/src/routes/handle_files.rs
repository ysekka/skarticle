use std::io::Write;

use actix_web as aw;

use crate::state::r#struct as stcState;

#[aw::post("/file/upload/{file_name}")]
pub async fn upload_file(app_state: aw::web::Data<stcState::State>, file_name: String, file_content: aw::web::Bytes) -> impl aw::Responder {
    let file_path = std::path::Path::new(app_state.public_directory.as_str())
    .join(file_name);

    let file = std::fs::File::create(file_path);

    if let Ok(mut file) = file {
        if let Ok(_) = file.write_all(&file_content) {
            return aw::HttpResponse::Ok().finish()
        }
    }

    aw::HttpResponse::BadRequest().finish()
}

#[aw::delete("/file/remove/{file_name}")]
pub async fn remove_file(app_state: aw::web::Data<stcState::State>, file_name: String) -> impl aw::Responder {
    let file_path = std::path::Path::new(app_state.public_directory.as_str())
    .join(file_name);

    if let Ok(_) = std::fs::remove_file(file_path) {
        return aw::HttpResponse::Ok().finish()
    }

    aw::HttpResponse::InternalServerError().finish()
}