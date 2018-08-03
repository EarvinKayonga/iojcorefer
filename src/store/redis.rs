use redis::Commands;

use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;

use std::collections::HashMap;
use std::string::String;

use failure::Error;

use super::super::models::{calculate_hash, Entry};
use super::Store;

impl Store for RedisStore {
    fn add_entry(&self, entry: Entry) -> Result<u64, Error> {
        let connection = self.get_connection()?;
        let hash = calculate_hash(&entry);

        if let Some(author) = entry.author {
            connection.hset_multiple(
                hash.to_string().as_str(),
                &[
                    ("link", entry.link.as_str()),
                    ("author", author.to_string().as_str()),
                ],
            )?;
        } else {
            connection.hset_multiple(hash.to_string().as_str(), &[("link", entry.link.as_str())])?;
        }

        Ok(hash)
    }

    fn fetch_entry(&self, hash: u64) -> Result<Option<Entry>, Error> {
        let connection = self.get_connection()?;

        let record: HashMap<String, String> = connection
            .hgetall(hash.to_string().as_str())
            .map_err(|err| {
                format_err!(
                    "an error occured while fetching struct from store {:?}",
                    err
                )
            })?;

        if let Some(link) = record.get("link") {
            let author: Option<u64> = record
                .get("author")
                .map(|id| id.parse())
                .map_or(Ok(None), |id| id.map(Some))?;

            Ok(Some(Entry {
                link: link.clone(),
                author: author,
            }))
        } else {
            Ok(None)
        }
    }

    fn delete_entry(&self, hash: u64) -> Result<(), Error> {
        let connection = self.get_connection()?;

        connection
            .hdel(hash, "link")
            .map_err(|err| format_err!("an error occured while deleting link field {:?}", err))?;

        connection
            .hdel(hash, "author")
            .map_err(|err| format_err!("an error occured while deleting author field {:?}", err))
    }
}

impl RedisStore {
    fn get_connection(&self) -> Result<PooledConnection<RedisConnectionManager>, Error> {
        self.pool
            .get()
            .map_err(|err| format_err!("an error occured while connecting to redis {:?}", err))
    }
}

#[derive(Clone, Debug)]
pub struct RedisStore {
    pub pool: Pool<RedisConnectionManager>,
}

pub fn new_redis_store(url: String) -> Result<RedisStore, Error> {
    debug!("connecting to [{}]", url.clone());

    let manager = RedisConnectionManager::new(url.clone().as_str()).map_err(|err| {
        format_err!(
            "an error occured while creating redis connection manager{:?}",
            err
        )
    })?;

    let pool: Pool<RedisConnectionManager> = Pool::builder()
        .max_size(3)
        .build(manager)
        .map_err(|err| format_err!("an error occured while creating redis pool {:?}", err))?;

    let _ping = pool
        .get()
        .map_err(|err| format_err!("an error occured while connecting to redis {:?}", err))?;

    info!("connection with redis [{}] is live", url.clone());

    Ok(RedisStore { pool: pool })
}
