use std::error::Error;

use lembas_api;
fn main() -> Result<(), Box<dyn Error>> {
    lembas_api::start()
}
