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





see rook/README.md






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



helm --namespace harbor uninstall harbor
kubectl delete namespace harbor




















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