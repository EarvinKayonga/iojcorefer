use actix_web::http::Method;
use actix_web::{fs, server, App};
use failure::Error;
use num_cpus::get as cpus;
use std::sync::Arc;

use crate::configuration::Configuration;
use crate::store::Store;

mod errors;
mod middlewares;
mod types;

// !! Context holds important values for the server.
#[derive(Clone)]
pub struct Context<T: 'static + Store + Send + Sync + Clone> {
    pub configuration: Arc<Configuration>,
    pub store: Arc<T>,
}

pub fn server<T: 'static + Store + Send + Sync + Clone>(context: Context<T>) -> Result<(), Error> {
    let state = context.clone();

    let sys = server::new(move || {
        let context = Arc::new(context.clone());
        let state = context.clone();

        let folder =
            fs::StaticFiles::new(state.clone().configuration.clone().server.folder.as_str())
                .expect("missing static folder");

        App::with_state(state)
            .resource("/api/v1/entries/new", |r| {
                r.method(Method::POST).f(middlewares::post_entry)
            })
            .resource("/api/v1/entry/{id}", |r| {
                r.method(Method::GET).a(middlewares::get_entry)
            })
            .resource("/api/v1/health", |r| {
                r.method(Method::GET).f(middlewares::health)
            })
            .handler("/", folder)
    })
    .shutdown_timeout(10) // <- Set shutdown timeout to 10 seconds
    .workers(workers_num());

    let socket = format!(
        "{}:{}",
        state.clone().configuration.clone().server.address,
        state.clone().configuration.clone().server.port
    );

    info!(
        "listening on port: {:?}",
        state.clone().configuration.clone().server.port
    );

    let reactor = sys
        .bind(socket.as_str())
        .map_err(|err| format_err!("an error occured while setting server {:?}", err))?;

    Ok(reactor.run())
}

fn workers_num() -> usize {
    cpus() / 2
}
