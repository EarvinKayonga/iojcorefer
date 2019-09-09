use env_logger::init as log_init;
use failure::Error;
use std::fs;
use std::sync::Arc;

use super::configuration::configuration as read_configuration;
use super::server::{server, Context};
use super::store::redis::new_redis_store;

pub fn run_app() -> Result<(), Error> {
    log_init();

    let config = read_configuration()
        .map_err(|e| format_err!("an error occured while reading configuration: {}", e))?;

    let _ = are_static_files_present(&config.clone().server.folder)
        .map_err(|e| format_err!("an error occured while opening static files folder {}", e))?;

    let redis = new_redis_store(config.clone().redis_address)
        .map_err(|e| format_err!("an error occured while connecting to store: {}", e))?;

    server(Context {
        configuration: Arc::new(config),
        store: Arc::new(redis),
    })
}

fn are_static_files_present(path: &str) -> Result<(), Error> {
    fs::read_dir(path)
        .map(|paths| {
            for entry in paths {
                match entry {
                    Ok(f) => debug!("static files: {}", f.path().display()),
                    Err(e) => debug!("an error occured while reading a file: {}", e),
                }
            }

            Ok(())
        })
        .map_err(|err| format_err!("an error occured while opening {}: {}", path, err))?
}
