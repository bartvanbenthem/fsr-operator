use k8s_openapi::api::core::v1::{
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeSpec, PersistentVolumeClaimSpec,
    VolumeResourceRequirements,
};
use k8s_openapi::api::storage::v1::StorageClass;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBundle {
    pub storage_classes: Vec<StorageClass>,
    pub persistent_volumes: Vec<PersistentVolume>,
    pub persistent_volume_claims: Vec<PersistentVolumeClaim>,
}

impl StorageBundle {
    pub fn new() -> Self {
        Self {
            storage_classes: Vec::new(),
            persistent_volumes: Vec::new(),
            persistent_volume_claims: Vec::new(),
        }
    }

    pub fn add_storage_class(&mut self, sc: StorageClass) {
        self.storage_classes.push(sc);
    }

    pub fn add_persistent_volume(&mut self, pv: PersistentVolume) {
        self.persistent_volumes.push(pv);
    }

    pub fn add_persistent_volume_claim(&mut self, pvc: PersistentVolumeClaim) {
        self.persistent_volume_claims.push(pvc);
    }

    pub fn dummy() -> Self {
        let mut bundle = Self::new();

        // -------- StorageClass --------
        let sc = StorageClass {
            metadata: ObjectMeta {
                name: Some("fast-storage".into()),
                ..Default::default()
            },
            provisioner: "example.com/dummy".into(),
            ..Default::default()
        };
        bundle.add_storage_class(sc);

        // -------- PersistentVolume --------
        let pv = PersistentVolume {
            metadata: ObjectMeta {
                name: Some("pv-fast-001".into()),
                ..Default::default()
            },
            spec: Some(PersistentVolumeSpec {
                storage_class_name: Some("fast-storage".into()),
                capacity: Some(
                    [("storage".into(), Quantity("10Gi".into()))]
                        .into_iter()
                        .collect(),
                ),
                access_modes: Some(vec!["ReadWriteOnce".into()]),
                ..Default::default()
            }),
            ..Default::default()
        };
        bundle.add_persistent_volume(pv);

        // -------- PersistentVolumeClaim --------
        let pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some("pvc-fast-claim".into()),
                namespace: Some("default".into()),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                storage_class_name: Some("fast-storage".into()),
                access_modes: Some(vec!["ReadWriteOnce".into()]),
                resources: Some(VolumeResourceRequirements {
                    requests: Some(
                        [("storage".into(), Quantity("10Gi".into()))]
                            .into_iter()
                            .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        bundle.add_persistent_volume_claim(pvc);

        bundle
    }
}
