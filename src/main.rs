mod logger;
mod color;
mod environment;
mod routes;
mod utils;

use std::env;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::shield::Shield;
use rocket_cors::AllowedHeaders;
use crate::environment::{check_env, load_dotenv};
use crate::logger::{log_banner, setup_logger};
use crate::utils::indexed_images::index_image_sizes;

#[macro_use]
extern crate rocket;

#[allow(clippy::print_stderr)]
fn setup_panic_hook() {
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\n============================================================");
        eprintln!("La Cabra Image API has panicked. This is a bug, please report this");
        eprintln!("at {}.", env!("CARGO_PKG_REPOSITORY"));
        eprintln!("If you can reliably reproduce this panic, include the");
        eprintln!("reproduction steps and re-run with the RUST_BACKTRACE=1 env");
        eprintln!("var set and include the backtrace in your report.");
        eprintln!();
        eprintln!("Platform: {} {}", env::consts::OS, env::consts::ARCH);
        eprintln!("Boot args: {:?}", env::args().collect::<Vec<_>>());
        eprintln!();
        orig_hook(panic_info);
    }));
}

#[launch]
async fn launch() -> _ {
    setup_logger().unwrap();
    if load_dotenv() {
        info!("Loaded environment variables from .env file");
    } else {
        warn!("Failed to load environment variables from .env file, make sure you have the required environment variables set")
    }
    check_env();

    if cfg!(debug_assertions) {
        warn!("Running in debug mode, optimizations are disabled and performance/speed might be affected");
    } else {
        setup_panic_hook();
    }

    log_banner();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: if cfg!(debug_assertions) {
            rocket_cors::AllowedOrigins::all()
        } else {
            rocket_cors::AllowedOrigins::some_regex(&[r"https?://(.+\.)?lacabra\.app"])
        },
        allowed_methods: vec![Method::Get, Method::Post, Method::Options, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]), // AllowedHeaders::All
        allow_credentials: true,
        expose_headers: ["Content-Type"].iter().map(|s| s.to_string()).collect(),
        max_age: Some(3600),
        ..Default::default()
    }.to_cors().expect("Error while building CORS");

    let port = env::var("PORT").unwrap_or("8000".to_string()).parse::<u16>().unwrap();
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());

    let indexed_image_sizes = index_image_sizes();
    info!("Indexed {} image types", indexed_image_sizes.len());
    for (key, value) in &indexed_image_sizes {
        info!(" - Found {} images of type '{}'", value.len(), key);
    }
    
    let domain_config = utils::domain::DomainConfig {
        domain: env::var("DOMAIN").unwrap_or("localhost".to_string()),
        is_https: env::var("USE_HTTPS").unwrap_or("false".to_string()).parse::<bool>().unwrap(),
    };

    rocket::build()
        .configure(rocket::Config {
            port,
            address: host.parse().unwrap(),
            ..Default::default()
        })
        .manage(indexed_image_sizes)
        .manage(domain_config)
        .mount("/assets", FileServer::from("assets"))
        .mount("/find", routes::find())
        .mount("/", routes::index())
        .register("/", routes::catchers())
        .attach(Shield::default())
        .attach(cors)
}
