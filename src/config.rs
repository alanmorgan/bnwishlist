use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct WishlistConfig {
    pub url: String
}

impl ::std::default::Default for WishlistConfig {
    fn default() -> Self { Self {url: "".into() } }
}

pub fn read_config() -> Option<WishlistConfig> {
    match confy::load_path(Path::new("wishlist.config")) {
        Ok(cfg) => Some(cfg),
        Err(_) => None
    }
}
