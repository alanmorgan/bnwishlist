use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Predicate, Text};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod book;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match config::read_config() {
        None => {
            println!("Invalid configuration file");
            return;
        }
        Some(cfg) => cfg,
    };

    let save_data: bool = args.len() == 2 && args[1].eq("--save");
    let read_from_file: bool = args.len() == 2 && !save_data;

    let txt = if read_from_file {
        get_wishlist_from_file(&args[1])
    } else {
        get_wishlist_from_http(&cfg.url)
    };

    if save_data && save_wishlist(&txt).is_err() {
        println!("Unable to save file");
        return;
    }

    if process_wishlist(&txt).is_err() {
        println!("Unable to process wishlist");
    }
}

fn save_wishlist(txt: &String) -> std::io::Result<()> {
    let mut file = File::create("wishlist.html")?;
    file.write_all(txt.as_bytes())?;

    Ok(())
}

fn process_wishlist(txt: &String) -> std::io::Result<()> {
    // Retrieve the new book list
    let document = Document::from_read(txt.as_bytes()).unwrap();

    let book_list = document
        .find(Class("prod-details-sec"))
        .map(build_book)
        .collect::<Vec<_>>();

    // Load and compare with old one
    let old_book_list = load_books();

    let price_change = find_changed_prices(&old_book_list, &book_list);

    // Show any books that have had a price change and then all books
    // selling at a dicsount
    if !price_change.is_empty() {
        println!("Price change");
        println!("============");

        for (book, old_price) in price_change {
            println!("{}, was {}", book, old_price);
        }

        println!("\n\n");
    }

    println!("Discounted");
    println!("==========");
    book_list
        .iter()
        .filter(|book| book.has_discount())
        .for_each(|book| println!("{}", book));

    save_books(book_list)?;

    Ok(())
}

fn extract_text(node: Node, pred: impl Predicate) -> Option<String> {
    node.find(pred).next().map(get_text)
}

/*
 * Each node contains text, but there may be excess spaces at the beginning
 * and end and carriage returns and nbsp. Collapse multiple spaces to single
 * space and trim junk from beginning and end
 */
fn get_text(node: Node) -> String {
    let re: Regex = Regex::new(r"(\n|\u{A0}| )+").unwrap();

    re.replace_all(
        &node
            .find(Text)
            .map(|t| t.text())
            .collect::<Vec<_>>()
            .join(""),
        " ",
        )
        .into_owned().trim().to_string()
}

fn get_wishlist_from_file(filename: &String) -> String {
    println!("Reading file {}", filename);
    match fs::read_to_string(filename) {
        Ok(txt) => txt,
        Err(_e) => panic!("Can't read file {}", filename),
    }
}

fn get_wishlist_from_http(url: &String) -> String {
    println!("Making HTTP request from {}\n\n", url);

    match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(txt) => txt,
            Err(_e) => {
                panic!("Can't read text from response")
            }
        },
        Err(_e) => {
            panic!("Can't read from url")
        }
    }
}

fn build_book(node: Node) -> book::Book {
    book::Book::new(
        extract_text(node, Class("product-shelf-title")).unwrap(),
        extract_text(node, Class("product-shelf-author"))
            .unwrap()
            .trim_start_matches("By: ")
            .to_string(),
        extract_text(node, Class("current-price")).unwrap(),
        extract_text(node, Class("discount-amount-text")),
    )
}

const JSON_FILENAME: &str = "wishlist.json";

fn save_books(books: Vec<book::Book>) -> std::io::Result<()> {
    let mut file = File::create(JSON_FILENAME)?;
    file.write_all(serde_json::to_string(&books).unwrap().as_bytes())?;

    Ok(())
}

fn load_books() -> Vec<book::Book> {
    match fs::read_to_string(JSON_FILENAME) {
        Ok(txt) => serde_json::from_str(&txt).unwrap(),
        Err(_e) => Vec::new(),
    }
}

fn find_changed_prices<'a>(
    old: &'a Vec<book::Book>,
    current: &'a [book::Book],
) -> Vec<(&'a book::Book, String)> {
    let mut changed = Vec::new();

    // This isn't particularly efficient, but does it matter?

    for old_book in old {
        let matching_books: Vec<&book::Book> = current
            .iter()
            .filter(|b| b.title.eq(&old_book.title))
            .collect::<Vec<&book::Book>>();

        if !matching_books.is_empty() {
            if matching_books.len() != 1 {
                println!("Bizzare. We have multiple books with the same title:");
                for book in &matching_books {
                    println!("{}", book)
                }
            }

            if !matching_books[0].current_price.eq(&old_book.current_price) {
                changed.push((matching_books[0], old_book.current_price.clone()));
            }
        }
    }

    changed
}
