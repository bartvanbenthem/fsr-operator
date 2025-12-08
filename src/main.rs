mod storage; // Import the storage module

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load the environment file once at startup.
    dotenvy::dotenv().ok();

    // 2. Define configuration parameters from environment variables (including container name).
    let container_name = env::var("AZURE_STORAGE_CONTAINER")
        .map_err(|_| anyhow::anyhow!("AZURE_STORAGE_CONTAINER must be set"))?;
    
    let target_path = "clustername/azure_v0_12_4_test.txt";
    let test_data = b"Data uploaded using reusable Rust storage functions.";

    // 3. Initialize the Object Store using the reusable library function.
    let store = storage::initialize_azure_store(&container_name)?;

    // 4. Perform the write and verification using the reusable library function.
    storage::write_data(store, target_path, test_data).await?;

    println!("Application completed successfully.");
    Ok(())
}