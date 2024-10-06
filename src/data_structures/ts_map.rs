use crate::data_structures::time_series::TimeSeries;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
struct CaseInsensitiveString(String);

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.0.to_lowercase().as_bytes() {
            state.write_u8(*byte);
        }
    }
}

pub struct TSMap {
    map: RwLock<HashMap<CaseInsensitiveString, Arc<RwLock<TimeSeries>>>>,
}

impl TSMap {
    pub fn new() -> Self {
        TSMap {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, key: String, initial_timestamp: u64, initial_value: f64) {
        let ts = TimeSeries::new(key.clone(), initial_timestamp, initial_value);
        let ts = Arc::new(RwLock::new(ts));
        let ci_key = CaseInsensitiveString(key);
        let mut map_write = self.map.write().unwrap();
        map_write.insert(ci_key, ts);
    }

    pub fn get(&self, key: &str) -> Option<Arc<RwLock<TimeSeries>>> {
        let map_read = self.map.read().unwrap();
        let ci_key = CaseInsensitiveString(key.to_string());
        map_read.get(&ci_key).cloned()
    }

    pub fn scan(&self) -> Vec<Arc<RwLock<TimeSeries>>> {
        let map_read = self.map.read().unwrap();
        map_read.values().cloned().collect()
    }
}
