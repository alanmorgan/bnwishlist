use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub current_price: String,
    pub discount_percentage: Option<String>,
}

impl Book {
    pub fn new(
        title: String,
        author: String,
        current_price: String,
        discount_percentage: Option<String>,
    ) -> Book {
        Book {
            title,
            author,
            current_price,
            discount_percentage,
        }
    }

    pub fn has_discount(&self) -> bool {
        self.discount_percentage.is_some()
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_discount() {
            write!(
                f,
                "{}, {}. Currently {} ({})",
                self.title,
                self.author,
                self.current_price,
                self.discount_percentage.as_ref().unwrap()
            )
        } else {
            write!(
                f,
                "{}, {}. Currently {}",
                self.title, self.author, self.current_price
            )
        }
    }
}
