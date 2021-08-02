with https://k3s.io/

for later: https://rancher.com/docs/k3s/latest/en/installation/private-registry/
for later: https://kubernetes.io/docs/tasks/debug-application-cluster/audit/
for later: https://kubernetes.io/docs/concepts/cluster-administration/logging/
for later: https://kubernetes.io/docs/tasks/debug-application-cluster/monitor-node-health/

follow https://rancher.com/docs/k3s/latest/en/installation/kube-dashboard/

follow https://rancher.com/docs/k3s/latest/en/cluster-access/

for later https://docs.fluentd.org/container-deployment/kubernetes
hints: https://medium.com/kubernetes-tutorials/cluster-level-logging-in-kubernetes-with-fluentd-e59aa2b6093a





vitess is pissing me off - skipping it and maybe use a static mysql cluster?
potentially use https://vitess.io/ at some point to learn something new but this is so impressively overkill...

git clone https://github.com/vitessio/vitess.git
cd vitess/examples/operator
kubectl apply -f operator.yaml
#kubectl apply -f 101_initial_cluster.yaml

kubectl apply -f vtorc_example.yaml












https://rook.io/docs/rook/v1.6/ceph-quickstart.html
git clone --single-branch --branch v1.6.8 https://github.com/rook/rook.git
cd rook/cluster/examples/kubernetes/ceph
kubectl create -f crds.yaml -f common.yaml -f operator.yaml
#kubectl create -f cluster-test.yaml # don't do this this is probably fucking dangerous as it's trying to use all devices
#kubectl create -f cluster.yaml
kubectl -n rook-ceph get pod

https://rook.io/docs/rook/v1.6/ceph-osd-mgmt.html
https://rook.io/docs/rook/v1.6/ceph-cluster-crd.html
\# GPT is not supported as disk format
kubectl create -f rook/host-based-cluster.yaml




https://rook.io/docs/rook/v1.6/ceph-toolbox.html
kubectl create -f rook/toolbox.yaml 
kubectl -n rook-ceph rollout status deploy/rook-ceph-tools
kubectl -n rook-ceph exec -it deploy/rook-ceph-tools -- bash


ceph status
ceph osd status
ceph df

kubectl -n rook-ceph delete deploy/rook-ceph-tools


https://rook.io/docs/rook/v1.6/ceph-dashboard.html



https://rook.io/docs/rook/v1.0/ceph-teardown.html#troubleshooting

rm -R /var/lib/rook




git clone https://github.com/mysql/mysql-operator
kubectl apply -f https://raw.githubusercontent.com/mysql/mysql-operator/trunk/deploy/deploy-crds.yaml
kubectl apply -f https://raw.githubusercontent.com/mysql/mysql-operator/trunk/deploy/deploy-operator.yaml
kubectl get deployment -n mysql-operator mysql-operator

kubectl apply -f mysql-operator/secret.yaml
kubectl apply -f mysql-operator/cluster.yaml
kubectl get innodbcluster --watch
kubectl get service mycluster
kubectl describe service mycluster
kubectl port-forward service/mycluster mysql
mysql -h127.0.0.1 -P6446 -uroot -p

kubectl get pods -l mysql.oracle.com/cluster-role=SECONDARY
kubectl get pods -l mysql.oracle.com/cluster-role=PRIMARY

you need a primary key btw. (https://dev.mysql.com/doc/refman/5.7/en/group-replication-requirements.html)

readonly:
kubectl port-forward service/mycluster mysql-ro
mysql -h127.0.0.1 -P6447 -uroot -p

so now I know that this works more or less. failover is a bit slow but well.

SELECT @@hostname;