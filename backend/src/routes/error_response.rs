use serde_json as sj;
use actix_web as aw;

pub fn error_response<B>(service_response: aw::dev::ServiceResponse) -> aw::Result<aw::middleware::ErrorHandlerResponse<B>> {
    let status_code = service_response.status();
    let (http_request, _) = service_response.into_parts();

    let http_response = aw::HttpResponseBuilder::new(status_code)
    .json(sj::json!({
        "status": status_code.as_u16()
    }))
    .map_into_boxed_body()
    .map_into_right_body();

    Ok(aw::middleware::ErrorHandlerResponse::Response(aw::dev::ServiceResponse::new(http_request, http_response)))
}