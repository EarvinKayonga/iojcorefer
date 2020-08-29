use failure::Error;

use super::models::Entry;

pub mod redis;
// !! Store is the trait that holds the app's state.
pub trait Store {
    fn add_entry(&self, entry: Entry) -> Result<u64, Error>;
    fn fetch_entry(&self, id: u64) -> Result<Option<Entry>, Error>;
    fn delete_entry(&self, id: u64) -> Result<(), Error>;
}
