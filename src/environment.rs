use std::env;
use std::fs::File;
use dotenv::dotenv;

/**
 * Load the environment variables from the .env file.
 *
 * # Example
 *
 * ```
 * use core::environment::load_dotenv;
 *
 * let loaded = load_dotenv();
 * assert_eq!(loaded, true);
 * ```
 */
pub fn load_dotenv() -> bool {
    match dotenv() {
        Ok(_) => true,
        Err(_) => false
    }
}

/**
 * Check if the required environment variables are set.
 *
 * # Example
 *
 * ```
 * use core::environment::check_env;
 *
 * check_env();
 * ```
 */
pub fn check_env() {

    assert!(env::var("HOST").is_ok(), "PORT is missing on the environment variables, please check your setup");
    assert!(env::var("PORT").is_ok(), "PORT is missing on the environment variables, please check your setup");
    assert!(env::var("ENABLE_METRICS").is_ok(), "ENABLE_METRICS is missing on the environment variables, please check your setup");
    assert!(env::var("METRICS_PASSWD").is_ok(), "METRICS_PASSWD is missing on the environment variables, please check your setup");
    assert!(env::var("DOMAIN").is_ok(), "DOMAIN is missing on the environment variables, please check your setup");
    assert!(env::var("USE_HTTPS").is_ok(), "USE_HTTPS is missing on the environment variables, please check your setup");


    let info = os_info::get();
    if is_docker() {
        info!("Running on a docker container: {} {} {}", info.os_type(), info.version(), info.bitness())
    } else {
        info!("Running on a native environment: {} {} {}", info.os_type(), info.version(), info.bitness())
    }

    info!("Process ID: '{}' started on '{}' by user '{}'", std::process::id(), get_runner_path(), get_runner_user());
}

/**
 * Check if the application is running in a Docker container.
 *
 * # Example
 *
 * ```
 * use core::environment::is_docker;
 *
 * let docker = is_docker();
 * assert_eq!(docker, true);
 * ```
 */
pub fn is_docker() -> bool {
    env::var("DOCKER").is_ok() || File::open("/.dockerenv").is_ok()
}

/**
 * Get the path of the application runner.
 *
 * # Example
 *
 * ```
 * use core::environment::get_runner_path;
 *
 * let path = get_runner_path();
 * assert_eq!(path, "/usr/la_cabra");
 * ```
 */
pub fn get_runner_path() -> String {
    match env::current_exe() {
        Ok(path) => {
            path.into_os_string().into_string().unwrap_or_else(|_| String::from("Failed to convert path to string"))
        },
        Err(_) => String::from("Failed to get current executable path"),
    }
}

/**
 * Get the user that started the application.
 *
 * # Example
 *
 * ```
 * use core::environment::get_runner_user;
 *
 * let user = get_runner_user();
 * assert_eq!(user, "root");
 * ```
 */
pub fn get_runner_user() -> String {
    whoami::username()
}