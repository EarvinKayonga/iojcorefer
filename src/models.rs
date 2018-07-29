use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// !!  Entry represents an entry in database.
#[derive(Clone)]
pub struct Entry {
    pub link: String,
    pub author: Option<u64>,
}

impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.link.hash(state);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


