use std::error::Error;

use backend::api;

fn main() -> Result<(), Box<dyn Error>> {
    api::router::start()
}
