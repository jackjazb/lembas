use std::error::Error;

use lembas::api;

fn main() -> Result<(), Box<dyn Error>> {
    api::router::start()
}
