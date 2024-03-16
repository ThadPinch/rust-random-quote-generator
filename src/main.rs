use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    quote: String,
    author: String
}

use std::fs;

fn load_quotes() -> Vec<Quote> {
    let quotes_json = fs::read_to_string("quotes.json").expect("Could not read file");
    serde_json::from_str(&quotes_json).expect("Failed to parse JSON")
}

use rand::Rng;

fn main() {
    let quotes = load_quotes();

    loop {
        let random_index = rand::thread_rng().gen_range(0..quotes.len());
        let random_quote = &quotes[random_index];

        println!("\n\"{}\"", random_quote.quote);
        println!("- {}", random_quote.author);

        println!("\nGenerate another quote? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_lowercase() != "y" {
            break;
        }
    }
}
