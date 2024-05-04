# foodloader

This utility is designed to assemble a dataset of ingredients that can be used to assemble recipes in
the untitled recipe app. The dataset is scraped from Sainsbury's public API, then processed into a format useful for the app. The app can be run with `cargo run --release`.

The command line arguments `--scrape-only` and `--load-only` will skip the SQL generation and web scraping phase respectively. These can be passed using `cargo run --release -- --argument`

Results are written to `ingredients.json`, with an SQL script to insert the ingredients written to `ingredients.sql`

API responses are written to `responses.json`, in case the API disappears.
