use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub type Presence = Arc<Mutex<HashSet<i32>>>;

pub fn new_presence() -> Presence {
    Arc::new(Mutex::new(HashSet::new()))
} 