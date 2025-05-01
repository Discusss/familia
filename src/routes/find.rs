use std::collections::HashMap;
use rand::Rng;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::utils::domain::DomainConfig;
use crate::utils::response::UrlResponse;

#[get("/<animal>")]
pub fn find(animal: &str, indexes: &State<HashMap<String, Vec<String>>>, domain_config: &State<DomainConfig>) -> Result<Json<UrlResponse<'static>>, Status> {
    if let Some(urls) = indexes.get(animal) {

        let mut rng = rand::rng();
        let random_index = rng.random_range(0..urls.len());

        let domain = format!("http{}://{}", if domain_config.is_https { "s" } else { "" }, domain_config.domain);
        let random_url = format!("{}/assets/{}/{}", domain, animal, urls[random_index]);

        Ok(Json(UrlResponse { status: "200", url: random_url }))
    } else {
        Err(Status::NotFound)
    }
}