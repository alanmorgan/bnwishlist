use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::default::Default;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WishlistConfig {
    pub url: String,
    #[serde(default = "default_cheap")]
    pub cheap: i64,
}

fn default_cheap() -> i64 {
  5
}

pub fn read_config() -> Option<WishlistConfig> {
    match confy::load_path(Path::new("wishlist.config")) {
        Ok(cfg) => Some(cfg),
        Err(_) => None,
    }
}
