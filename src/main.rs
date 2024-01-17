// region : --- Modules
mod ais;
mod buddy;
mod error;

use crate::ais::asst::ENV_OPENAI_API_KEY;

pub use self::error::{Error, Result};

async fn start() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
#[tokio::main]
async fn main() {
    println!();
    println!("API Key test {}", ENV_OPENAI_API_KEY);

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\n Error: {} \n", e),
    }
}
