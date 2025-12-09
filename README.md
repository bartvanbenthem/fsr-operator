# pvsync
Kubernetes controller that logs the state of storage resources on a Kubernetes cluster. 

### upcoming release
Features in currently in development for the upcoming release:
* remove old logs based on a given retention time in days in the cr spec

## Build container
```bash
source ../00-ENV/env.sh
CVERSION="v0.6.1"

docker login ghcr.io -u bartvanbenthem -p $CR_PAT

docker build -t pvsync:$CVERSION .

docker tag pvsync:$CVERSION ghcr.io/bartvanbenthem/pvsync:$CVERSION
docker push ghcr.io/bartvanbenthem/pvsync:$CVERSION

# test image
docker run --rm -it --entrypoint /bin/sh pvsync:$CVERSION

/# ls -l /usr/local/bin/pvsync
/# /usr/local/bin/pvsync
```

## Deploy CRD
```bash
kubectl apply -f ./config/crd/pvsync.storage.cndev.nl.yaml
# kubectl delete -f ./config/crd/pvsync.storage.cndev.nl.yaml
```

## Deploy Operator
```bash
helm install pvsync ./config/operator/chart --create-namespace --namespace dr-operator
# helm -n dr-operator uninstall pvsync
```

## Sample tracker resource
```bash
kubectl -n dr-operator apply -f ./config/samples/pvsync-example.yaml
kubectl -n dr-operator describe persistentvolumesyncs.storage.cndev.nl example-pvsync
# kubectl -n dr-operator  delete -f ./config/samples/pvsync-example.yaml
```

## Test Watchers & Reconciler on Create Persistant Volumes
```bash
kubectl apply -f ./config/samples/test-pv.yaml
kubectl delete -f ./config/samples/test-pv.yaml
```

## CR Spec
```yaml
apiVersion: storage.cndev.nl/v1alpha1
kind: PersistentVolumeSync
metadata:
  name: example-pvsync
  labels:
    app.kubernetes.io/name: persistentvolumesync
    app.kubernetes.io/part-of: volumesync-operator
  annotations:
    description: "Tracks persistent volume usage and logs it"
spec:
  clusterNameKey: cluster.x-k8s.io/cluster-name # retreiving the cluster name by annotation key is required
  cloudProvider: azure # or s3
  retention: 14 # retention in days
```