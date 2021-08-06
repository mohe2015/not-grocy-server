with https://k3s.io/

currently after every startup: kubectl patch storageclass local-path -p '{"metadata": {"annotations":{"storageclass.kubernetes.io/is-default-class":"false"}}}'

for later: https://rancher.com/docs/k3s/latest/en/installation/private-registry/
for later: https://kubernetes.io/docs/tasks/debug-application-cluster/audit/
for later: https://kubernetes.io/docs/concepts/cluster-administration/logging/
for later: https://kubernetes.io/docs/tasks/debug-application-cluster/monitor-node-health/

follow https://rancher.com/docs/k3s/latest/en/installation/kube-dashboard/

follow https://rancher.com/docs/k3s/latest/en/cluster-access/

for later https://docs.fluentd.org/container-deployment/kubernetes
hints: https://medium.com/kubernetes-tutorials/cluster-level-logging-in-kubernetes-with-fluentd-e59aa2b6093a




https://nixos.wiki/wiki/Kubernetes
boot.kernelModules = [ "ceph" ];
sudo modprobe rbd



https://rook.io/docs/rook/v1.6/ceph-quickstart.html
git clone --single-branch --branch v1.6.8 https://github.com/rook/rook.git
cd rook/cluster/examples/kubernetes/ceph
kubectl create -f crds.yaml -f common.yaml -f operator.yaml
#kubectl create -f cluster-test.yaml # don't do this this is probably fucking dangerous as it's trying to use all devices
#kubectl create -f cluster.yaml
kubectl -n rook-ceph get pod

https://rook.io/docs/rook/v1.6/ceph-osd-mgmt.html
https://rook.io/docs/rook/v1.6/ceph-cluster-crd.html
\# GPT is not supported as disk format use MSDOS
kubectl create -f rook/host-based-cluster.yaml


https://kubernetes.io/docs/tasks/administer-cluster/change-default-storage-class/
kubectl get storageclass
kubectl patch storageclass local-path -p '{"metadata": {"annotations":{"storageclass.kubernetes.io/is-default-class":"false"}}}'
kubectl patch storageclass rook-ceph-block -p '{"metadata": {"annotations":{"storageclass.kubernetes.io/is-default-class":"true"}}}'


https://rook.io/docs/rook/v1.6/ceph-toolbox.html
kubectl create -f rook/toolbox.yaml 
kubectl -n rook-ceph rollout status deploy/rook-ceph-tools
kubectl -n rook-ceph exec -it deploy/rook-ceph-tools -- bash


ceph status
ceph osd status
ceph df

kubectl -n rook-ceph delete deploy/rook-ceph-tools


https://rook.io/docs/rook/v1.6/ceph-dashboard.html




https://rook.io/docs/rook/v1.6/ceph-object.html
this only seems to work with 3 nodes...
kubectl create -f docs/kubernetes/rook/object.yaml
kubectl -n rook-ceph get pod -l app=rook-ceph-rgw
kubectl create -f docs/kubernetes/rook/bucket.yaml
kubectl create -f docs/kubernetes/rook/bucket-claim.yaml



https://rook.io/docs/rook/v1.6/ceph-block.html
kubectl create -f rook/cluster/examples/kubernetes/ceph/csi/rbd/storageclass-test.yaml



https://rook.io/docs/rook/v1.6/ceph-filesystem.html
kubectl create -f rook/cluster/examples/kubernetes/ceph/filesystem-test.yaml
kubectl -n rook-ceph get pod -l app=rook-ceph-mds
kubectl create -f rook/cluster/examples/kubernetes/ceph/csi/cephfs/storageclass.yaml



https://rook.io/docs/rook/v1.0/ceph-teardown.html#troubleshooting

rm -R /var/lib/rook



















git clone https://github.com/vitessio/vitess.git
kubectl apply -f vitess/examples/operator/operator.yaml
#kubectl apply -f vitess/examples/operator/101_initial_cluster.yaml
kubectl apply -f vitess/examples/operator/vtorc_example.yaml

\# vtorc
kubectl port-forward deployment/example-commerce-x-x-zone1-vtorc-c13ef6ff 3001:3000
http://localhost:3001
\# this is epic - you can graphically see the hierarchy and promote primaries etc.

./vitess/examples/operator/pf.sh

http://localhost:15000/app/

nix shell nixpkgs#mariadb-client nixpkgs#go
go get vitess.io/vitess/go/cmd/vtctlclient

alias vtctlclient="~/go/bin/vtctlclient -server=localhost:15999"
alias mysql="mysql -h 127.0.0.1 -P 15306 -u user"

kubectl get pods

vtctlclient ApplySchema -sql="$(cat vitess/examples/operator/create_commerce_schema.sql)" commerce
vtctlclient ApplyVSchema -vschema="$(cat vitess/examples/operator/vschema_commerce_initial.json)" commerce

https://vitess.io/docs/user-guides/schema-changes/unmanaged-schema-changes/

https://github.com/github/gh-ost/blob/master/doc/why-triggerless.md














helm repo add harbor https://helm.goharbor.io
helm fetch harbor/harbor --untar
cd harbor
kubectl create namespace harbor
helm --namespace harbor install harbor .
\# wait
harbor ingress -> resource information
/etc/hosts from core.harbor.domain to that ip

username admin password Harbor12345

https://goharbor.io/docs/2.3.0/administration/configure-authentication/db-auth/

https://goharbor.io/docs/2.3.0/working-with-projects/create-projects/

sudo mkdir -p /etc/docker/certs.d/core.harbor.domain/
sudo cp ~/Downloads/core-harbor-domain-chain.pem /etc/docker/certs.d/core.harbor.domain/ca.crt

docker login https://core.harbor.domain/


docker build -t not-grocy-server .
docker tag not-grocy-server:latest core.harbor.domain/library/not-grocy-server
docker push core.harbor.domain/library/not-grocy-server

https://github.com/aquasecurity/trivy/issues/67
https://github.com/aquasecurity/trivy/issues/160

trivy --clear-cache
























https://github.com/ory/kratos
https://github.com/ory/k8s



https://www.keycloak.org/getting-started/getting-started-kube
\# also has an operator

wget -q -O - https://raw.githubusercontent.com/keycloak/keycloak-quickstarts/latest/kubernetes-examples/keycloak-ingress.yaml | \
sed "s/KEYCLOAK_HOST/keycloak.local/" | \
kubectl create -f -

username: admin
password: admin

http://keycloak.local/auth/realms/not-grocy/account/#/

add client scopes for all permissions

clients->client scopes->setup add client scopes to default

Consent Required

access type: confidential https://www.keycloak.org/docs/latest/authorization_services/
authorization enabled

authorization -> resources -> new -> stock


maybe use roles?


https://www.keycloak.org/app/

https://stackoverflow.com/questions/42186537/resources-scopes-permissions-and-policies-in-keycloak