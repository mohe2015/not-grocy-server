kubectl -n kubernetes-dashboard describe secret admin-user-token | grep '^token'
kubectl proxy

potentially use https://vitess.io/ at some point to learn something new but this is so impressively overkill...

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