use failure::Error;

use std::sync::Arc;

use env_logger::init as log_init;

use super::configuration::configuration as read_configuration;
use super::server::{server, Context};
use super::store::new_redis_store;

pub fn run_app() -> Result<(), Error> {
    log_init();

    let config = read_configuration()
        .map_err(|e| format_err!("an error occured while reading configuration: {}", e))?;

    let redis = new_redis_store(config.clone().redis_address)
        .map_err(|e| format_err!("an error occured while connecting to store: {}", e))?;

    server(Context {
        configuration: Arc::new(config.clone()),
        store: Arc::new(redis.clone()),
    })
}
