use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use crate::utils::response::{GenericResponse, GenericStringResponse};

#[catch(401)]
pub fn unauthorized() -> Json<GenericResponse<'static>> {
    let response_json = GenericResponse {
        status: "401",
        message: "Unauthorized",
    };
    Json(response_json)
}

#[catch(403)]
pub fn forbidden() -> Json<GenericResponse<'static>> {
    let response_json = GenericResponse {
        status: "403",
        message: "You are forbidden from accessing this resource",
    };
    Json(response_json)
}

#[catch(404)]
pub fn not_found() -> Json<GenericResponse<'static>> {
    let response_json = GenericResponse {
        status: "404",
        message: "Not found",
    };
    Json(response_json)
}

#[catch(429)]
pub fn too_many_requests() -> Json<GenericResponse<'static>> {
    let response_json = GenericResponse {
        status: "429",
        message: "You are being rate limited",
    };
    Json(response_json)
}

#[catch(default)]
pub fn internal_error(status: Status, _: &Request) -> Json<GenericStringResponse> {
    let response_json = GenericStringResponse {
        status: status.code.to_string(),
        message: status.reason().unwrap().to_string(),
    };
    Json(response_json)
}