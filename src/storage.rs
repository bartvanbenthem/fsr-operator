use anyhow::{anyhow, Result};
use object_store::azure::MicrosoftAzureBuilder;
use object_store::path::Path;
use object_store::{ObjectStore, PutPayload};
use bytes::Bytes;
use std::sync::Arc;
use std::env;

// --- 1. Object Store Builder ---

/// Initializes and returns an Arc to the Azure Blob ObjectStore.
/// 
/// It relies on environment variables (AZURE_STORAGE_ACCOUNT, AZURE_STORAGE_ACCESS_KEY) 
/// and requires the container name.
pub fn initialize_azure_store(container_name: &str) -> Result<Arc<dyn ObjectStore>> {
    // Note: We use env::var() here. The caller (main) is responsible for loading the .env file.
    let account = env::var("AZURE_STORAGE_ACCOUNT")
        .map_err(|_| anyhow!("AZURE_STORAGE_ACCOUNT not found in environment"))?;
    
    let access_key = env::var("AZURE_STORAGE_ACCESS_KEY")
        .map_err(|_| anyhow!("AZURE_STORAGE_ACCESS_KEY not found in environment"))?;

    println!("Attempting to connect to Azure Blob Storage...");
    
    let store = MicrosoftAzureBuilder::new()
        .with_account(account)
        .with_access_key(access_key)
        .with_container_name(container_name)
        .build()
        .map_err(|e| anyhow!("Failed to build Azure Store: {}", e))?;

    Ok(Arc::new(store))
}

// --- 2. Write Function ---

/// Writes a slice of bytes to a specified path in the object store.
pub async fn write_data(
    store: Arc<dyn ObjectStore>, 
    path: &str, 
    data: &[u8]
) -> Result<()> {
    let object_key = Path::from(path);
    let container_name = env::var("AZURE_STORAGE_CONTAINER")
        .unwrap_or_else(|_| "Unknown Container".to_string());

    let bytes_payload = Bytes::from(data.to_vec());
    let payload: PutPayload = bytes_payload.into(); // Use the Into trait for conversion
    
    println!("Writing data to container '{}' at path: {}", container_name, object_key);

    // 4. Perform the atomic write operation.
    let result = store.put(&object_key, payload).await
        .map_err(|e| anyhow!("Failed to put object: {}", e))?;

    println!("File written successfully!");
    println!("   Returned E-Tag: {:?}", result.e_tag);

    // 5. Verification (HEAD request).
    let metadata = store.head(&object_key).await
        .map_err(|e| anyhow!("Failed to read metadata: {}", e))?;
    println!("--- Verification ---");
    println!("Retrieved object size: {} bytes", metadata.size);
    
    // 6. Cleanup.
    //store.delete(&object_key).await
    //    .map_err(|e| anyhow!("Failed to delete object: {}", e))?;
    //println!("Object deleted successfully for cleanup.");

    Ok(())
}