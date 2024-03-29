use actix_web::{AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Json, Responder};
use failure::Error;
use futures::future::{result, Future, FutureResult};
use std::sync::Arc;

use super::errors;
use super::types;
use super::Context;
use crate::models::Entry;
use crate::store::Store;

pub fn health<T: 'static + Store + Send + Sync + Clone>(
    _: &HttpRequest<Arc<Context<T>>>,
) -> impl Responder {
    Json(types::Health {})
}

pub fn get_entry<T: 'static + Store + Send + Sync + Clone>(
    req: &HttpRequest<Arc<Context<T>>>,
) -> FutureResult<HttpResponse, errors::FetchError> {
    let id: u64 = match req.match_info().get("id").map(|id| id.parse()) {
        Some(Ok(id)) => id,
        Some(Err(_)) => return result(Err(errors::FetchError::IDParsingError)),
        None => return result(Err(errors::FetchError::NoProvidedIdError)),
    };

    let store = req.state().store.clone();

    let entry = match (*store).fetch_entry(id) {
        Ok(None) => Err(errors::FetchError::NotFoundError),
        Err(err) => {
            error!("an error occured while fetching entry {:?}", err);

            Err(errors::FetchError::StoreError)
        }
        Ok(Some(entry)) => {
            let response = types::Link { text: entry.link };
            Ok(HttpResponse::Ok().json(response))
        }
    };

    result(entry)
}

pub fn post_entry<T: 'static + Store + Send + Sync + Clone>(
    req: &HttpRequest<Arc<Context<T>>>,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let store = req.state().store.clone();

    req.json()
        .from_err()
        .and_then(move |entry: types::Link| {
            let id = (*store).add_entry(Entry {
                link: entry.text,
                author: None,
            })?;

            Ok(HttpResponse::Ok().json(types::Hash { hash: id }))
        })
        .responder()
}
