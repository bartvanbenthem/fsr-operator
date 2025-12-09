use kube::CustomResource;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[kube(
    group = "storage.example.com",
    version = "v1alpha1",
    kind = "PersistentVolumeSync",
    plural = "persistentvolumesyncs",
    status = "PersistentVolumeSyncStatus",
    derive = "Default",
)]
pub struct PersistentVolumeSyncSpec {
    pub mode: SyncMode,
    pub backend: BackendConfig,
    pub selector: Option<VolumeSelector>,
    pub schedule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncMode {
    Protected,
    Recovery,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BackendConfig {
    pub provider: String,
    pub bucket: String,
    pub prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VolumeSelector {
    pub storageClass: Option<String>,
    pub annotations: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PersistentVolumeSyncStatus {
    pub lastSyncTime: Option<String>,
    pub observedGeneration: Option<i64>,
    pub phase: Option<String>,
}
