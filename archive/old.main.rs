mod storage;

use anyhow::{Result, anyhow};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load the environment file once at startup.
    dotenvy::dotenv().ok();

    // Generate a unique timestamp for the file name
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // --- Core Logic: Select Provider and Initialize Store ---

    // 2. Determine the cloud provider from environment variable.
    let provider = env::var("CLOUD_PROVIDER")
        .map_err(|_| anyhow!("CLOUD_PROVIDER must be set (e.g., 'azure' or 's3')"))?
        .to_lowercase();

    // 3. Define common application parameters.
    let target_path = format!("clustername/{}_test_file.json", timestamp);

    // test data
    let test_storage_bundle = storage::StorageBundle::dummy();
    let test_data = serde_json::to_string_pretty(&test_storage_bundle)?;

    println!("Selected Provider: {}", provider);

    // 4. Initialize the Object Store dynamically.
    let store = match provider.as_str() {
        "azure" => {
            let container_name = env::var("OBJECT_STORAGE_BUCKET")
                .map_err(|_| anyhow::anyhow!("OBJECT_STORAGE_BUCKET must be set for Azure"))?;

            storage::initialize_azure_store(&container_name)?
        }
        "s3" => {
            // Note: For S3, the bucket name is required. We'll use AWS_BUCKET_NAME convention.
            let bucket_name = env::var("OBJECT_STORAGE_BUCKET")
                .map_err(|_| anyhow::anyhow!("OBJECT_STORAGE_BUCKET must be set for S3"))?;

            // Allow an optional custom endpoint for S3-compatible systems (like Cloudian or MinIO)
            let endpoint = env::var("S3_ENDPOINT_URL").ok();

            storage::initialize_s3_store(&bucket_name, endpoint.as_deref())?
        }
        _ => {
            anyhow::bail!(
                "Unsupported CLOUD_PROVIDER: {}. Use 'azure' or 's3'.",
                provider
            )
        }
    };

    // --- Store Operations (Cloud Agnostic) ---

    // 5. Perform the write and verification using the reusable library function.
    // This call works seamlessly with either the Azure or S3 store.
    storage::write_data(store, &target_path, &test_data.as_bytes()).await?;

    println!(
        "Application completed successfully. Object written to path: {}",
        target_path
    );

    Ok(())
}
