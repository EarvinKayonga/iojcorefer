use actix_web::{AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Json, Responder};
use failure::Error;
use futures::future::{result, Future, FutureResult};
use std::sync::Arc;

use super::errors;
use super::types;
use super::Context;

use super::super::models::Entry;

pub fn health(_: &HttpRequest<Arc<Context>>) -> impl Responder {
    Json(types::Health {})
}

pub fn get_entry(
    req: &HttpRequest<Arc<Context>>,
) -> FutureResult<HttpResponse, errors::FetchError> {
    let id: u64 = match req.match_info().get("id").map(|id| id.parse()) {
        Some(Ok(id)) => id,
        Some(Err(_)) => return result(Err(errors::FetchError::IDParsingError)),
        None => return result(Err(errors::FetchError::NoProvidedIdError)),
    };

    let store = req.state().store.clone();

    let entry = match (*store).fetch_entry(id) {
        Err(err) => {
            error!("an error occured while fetching entry {:?}", err);

            Err(errors::FetchError::StoreError)
        }
        Ok(option) => {
            if let Some(entry) = option {
                let response = types::Link { text: entry.link };
                Ok(HttpResponse::Ok().json(response))
            } else {
                Err(errors::FetchError::NotFoundError)
            }
        }
    };

    result(entry)
}

pub fn post_entry(
    req: &HttpRequest<Arc<Context>>,
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
