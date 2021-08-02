with https://k3s.io/

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



















vitess is pissing me off - skipping it and maybe use a static mysql cluster?
potentially use https://vitess.io/ at some point to learn something new but this is so impressively overkill...

git clone https://github.com/vitessio/vitess.git
kubectl apply -f vitess/examples/operator/operator.yaml
kubectl apply -f vitess/examples/operator/101_initial_cluster.yaml
#kubectl apply -f vitess/examples/operator/vtorc_example.yaml





















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