use rocket::serde::json::Json;
use crate::utils::response::GenericResponse;

#[get("/")]
pub fn index() -> Json<GenericResponse<'static>> {
    Json(GenericResponse {
        status: "200",
        message: "Hello, world!",
    })
}
