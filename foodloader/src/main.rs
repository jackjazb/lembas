use std::{env, error, iter};

mod loader;
mod scraper;

const LOAD_ONLY: &str = "--load-only";
const SCRAPE_ONLY: &str = "--scrape-only";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    print_bordered("foodloader v0.1");
    let args: Vec<String> = env::args().collect();

    if !args.contains(&String::from(LOAD_ONLY)) {
        scraper::scrape().await?;
    }
    if !args.contains(&String::from(SCRAPE_ONLY)) {
        loader::load().await?;
    }

    Ok(())
}

fn print_bordered(string: &str) {
    let len = string.len();
    let frame = format!("+{}+", iter::repeat("-").take(len + 2).collect::<String>());
    let contents = format!("| {} |", string);
    println!("{}", frame);
    println!("{}", contents);
    println!("{}", frame);
}
