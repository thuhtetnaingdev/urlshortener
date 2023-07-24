// src/shortener.rs

use crate::models::UrlMap;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct UrlShortener {
    url_map: Arc<RwLock<HashMap<String, UrlMap>>>,
}

impl UrlShortener {
    pub fn new() -> Self {
        UrlShortener {
            url_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn shorten_url(&self, long_url: &str) -> String {
        let short_url = self.generate_short_url();
        let url_map_entry = UrlMap {
            short_url: short_url.clone(),
            long_url: long_url.to_string(),
        };

        let mut url_map = self.url_map.write().unwrap();
        url_map.insert(short_url.clone(), url_map_entry);

        short_url
    }

    pub fn redirect_url(&self, short_url: &str) -> Option<String> {
        let url_map = self.url_map.read().unwrap();

        // let _ = url_map.clone().iter().map(|x| println!("{:?}", x));
        // println!("{:?}", url_map);
        url_map
            .get(short_url)
            .map(|url_map| url_map.long_url.clone())
    }

    fn generate_short_url(&self) -> String {
        const SHORT_URL_LENGTH: usize = 6;
        let mut rng = thread_rng();
        (0..SHORT_URL_LENGTH)
            .map(|_| rng.sample(Alphanumeric).to_string())
            .collect()
    }
}
