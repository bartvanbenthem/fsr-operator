# k8sfsr
Kubernetes controller that logs the state of storage resources on a Kubernetes cluster. 

### upcoming release
Features in currently in development for the upcoming release:
* remove old logs based on a given retention time in days in the cr spec

## Build container
```bash
source ../00-ENV/env.sh
CVERSION="v0.6.1"

docker login ghcr.io -u bartvanbenthem -p $CR_PAT

docker build -t k8sfsr:$CVERSION .

docker tag k8sfsr:$CVERSION ghcr.io/bartvanbenthem/k8sfsr:$CVERSION
docker push ghcr.io/bartvanbenthem/k8sfsr:$CVERSION

# test image
docker run --rm -it --entrypoint /bin/sh k8sfsr:$CVERSION

/# ls -l /usr/local/bin/k8sfsr
/# /usr/local/bin/k8sfsr
```

## Deploy CRD
```bash
kubectl apply -f ./config/crd/tracker.cndev.nl.yaml
# kubectl delete -f ./config/crd/tracker.cndev.nl.yaml
```

## Deploy Operator
```bash
helm install k8sfsr ./config/operator/chart --create-namespace --namespace dr-operator
# helm -n dr-operator uninstall k8sfsr
```

## Sample tracker resource
```bash
kubectl -n dr-operator apply -f ./config/samples/tracker-example.yaml
kubectl -n dr-operator describe volumetrackers.cndev.nl example-tracker
# kubectl -n dr-operator  delete -f ./config/samples/tracker-example.yaml
```

## Test Watchers & Reconciler on Create Persistant Volumes
```bash
kubectl apply -f ./config/samples/test-pv.yaml
kubectl delete -f ./config/samples/test-pv.yaml
```

## CR Spec
```yaml
apiVersion: cndev.nl/v1beta1
kind: VolumeTracker
metadata:
  name: example-tracker
  namespace: default
  labels:
    app.kubernetes.io/name: volumetracker
    app.kubernetes.io/part-of: disaster-recovery-operator
  annotations:
    description: "Tracks persistent volume usage and logs it"
spec:
  clusterNameKey: cluster.x-k8s.io/cluster-name # retreiving the cluster name by annotation key is required
  cloudProvider: azure # or s3
  retention: 14 # retention in days
```