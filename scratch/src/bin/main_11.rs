use std::time::Duration;

use tokio::time::sleep;
use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Waiting...");
    sleep(Duration::from_secs(5)).await;
    println!("Done waiting!");
    Ok(())
}
