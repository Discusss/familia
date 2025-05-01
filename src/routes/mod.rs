mod catchers;
mod index;
mod find;

use crate::routes::catchers::*;

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![
        not_found,
        internal_error,
        forbidden,
        unauthorized,
        too_many_requests
    ]
}

pub fn index() -> Vec<rocket::Route> {
    routes![
        index::index,
    ]
}

pub fn find() -> Vec<rocket::Route> {
    routes![
        find::find,
    ]
}