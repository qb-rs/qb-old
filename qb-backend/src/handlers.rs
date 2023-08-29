use actix_web::{
    dev::ServiceResponse, error::JsonPayloadError, middleware::ErrorHandlerResponse, HttpRequest,
    Result,
};
use http_api_problem::{HttpApiProblem, StatusCode};

use crate::prob::*;

pub fn handle_not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();

    let res = HttpApiProblem::new(StatusCode::NOT_FOUND)
        .title("Not found")
        .detail("The requested resource could not be found.")
        .instance(req.uri().path())
        .type_url(NOT_FOUND)
        .to_actix_response();

    let res = ServiceResponse::new(req, res).map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
//for<'a> fn(JsonPayloadError, &'a HttpRequest) -> _
pub fn handle_json_error<'a>(err: JsonPayloadError, req: &'a HttpRequest) -> actix_web::Error {
    let mut prob = HttpApiProblem::new(StatusCode::UNPROCESSABLE_ENTITY)
        .title("Invalid json payload")
        .detail("An error occured while processing the json payload.")
        .type_url(INVALID_PAYLOAD)
        .instance(req.uri().path());
    prob.set_value("error", &format!("{err}"));

    let res = prob.to_actix_response();

    actix_web::error::InternalError::from_response("", res).into()
}
