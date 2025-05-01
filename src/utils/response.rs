use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse<'a> {
    pub status: &'a str,
    pub message: &'a str,
}

#[derive(Serialize)]
pub struct GenericStringResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct UrlResponse<'a> {
    pub status: &'a str,
    pub url: String,
}