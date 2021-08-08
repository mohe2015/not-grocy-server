# https://kubernetes.io/docs/reference/setup-tools/kubeadm/
# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/
# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/

# install hcloud https://github.com/hetznercloud/cli
hcloud context create kubernetes

create three servers of type cpx11 (min 40GB disk)
# hcloud server create --type cpx11 --image debian-10 --ssh-key moritz@nixos --name node-1 --datacenter nbg1-dc3
# hcloud server create --type cpx11 --image debian-10 --ssh-key moritz@nixos --name node-2 --datacenter hel1-dc2
# hcloud server create --type cpx11 --image debian-10 --ssh-key moritz@nixos --name node-3 --datacenter fsn1-dc14
hcloud server enable-protection kubernetes-node-1 delete rebuild
hcloud server enable-protection kubernetes-node-2 delete rebuild
hcloud server enable-protection kubernetes-node-3 delete rebuild


later: use load balancer, now: add dns to node-1 kube-apiserver.selfmade4u.de
https://github.com/kubernetes/kubeadm/blob/master/docs/ha-considerations.md#keepalived-and-haproxy

https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/

ssh root@kubernetes-node-x.selfmade4u.de



sudo nano /etc/containerd/config.toml
[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc]
  ...
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc.options]
    SystemdCgroup = true

sudo systemctl restart containerd

# installing kubeadm, kubelet, kubectl

# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/#stacked-control-plane-and-etcd-nodes

# https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/
# https://pkg.go.dev/k8s.io/kubelet/config/v1beta1?utm_source=godoc#KubeletConfiguration

# on failure:
kubeadm reset
iptables -F && iptables -t nat -F && iptables -t mangle -F && iptables -X
rm -R /etc/cni/net.d

# if already joined cluster:
# remove node using kubectl delete node to try again
# also remove from etcd (if automatic removal failed):
https://kubernetes.io/docs/tasks/administer-cluster/configure-upgrade-etcd/
kubectl get pods --namespace kube-system -o wide | grep etcd
kubectl exec etcd-kubernetes-node-1 -n kube-system -- etcdctl --cacert /etc/kubernetes/pki/etcd/ca.crt --key /etc/kubernetes/pki/etcd/server.key --cert /etc/kubernetes/pki/etcd/server.crt  --endpoints=23.88.58.221:2379,23.88.39.133:2379 member list
kubectl exec etcd-kubernetes-node-1 -n kube-system -- etcdctl --cacert /etc/kubernetes/pki/etcd/ca.crt --key /etc/kubernetes/pki/etcd/server.key --cert /etc/kubernetes/pki/etcd/server.crt  --endpoints=23.88.58.221:2379,23.88.39.133:2379 member remove e5c87eae083faedd


export KUBECONFIG=/etc/kubernetes/admin.conf

# only required on first node afaik
kubectl apply -f https://raw.githubusercontent.com/coreos/flannel/master/Documentation/kube-flannel.yml

kubectl get pod -n kube-system -w

kubectl logs -n kube-system kube-flannel-ds-6z5cf


scp root@kubernetes-node-1.selfmade4u.de:/etc/kubernetes/admin.conf .

export KUBECONFIG=$HOME/admin.conf

kubectl get nodes
kubectl proxy


# https://github.com/kubernetes/dashboard
kubectl apply -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.3.1/aio/deploy/recommended.yaml

# https://github.com/kubernetes/dashboard/blob/master/docs/user/access-control/creating-sample-user.md

kubectl apply -f dashboard-adminuser.yaml

kubectl -n kubernetes-dashboard get secret $(kubectl -n kubernetes-dashboard get sa/admin-user -o jsonpath="{.secrets[0].name}") -o go-template="{{.data.token | base64decode}}"



https://github.com/cncf/k8s-conformance/blob/master/instructions.md
# this only works with a non-master node
sonobuoy run --mode=certified-conformance
sonobuoy status
sonobuoy logs
outfile=$(sonobuoy retrieve)
sonobuoy delete


# TODO storage classes, ingresses



export KUBECONFIG=$HOME/admin.conf
# or
cp $HOME/admin.conf ~/.kube/config


dig kube-apiserver.selfmade4u.de


# maintenance
kubectl drain kubernetes-node-1 --ignore-daemonsets --delete-emptydir-data
# do maintenance
kubectl uncordon kubernetes-node-1

kubectl taint nodes kubernetes-node-1 node-role.kubernetes.io/master:NoSchedule-
kubectl taint nodes kubernetes-node-2 node-role.kubernetes.io/master:NoSchedule-
kubectl taint nodes kubernetes-node-3 node-role.kubernetes.io/master:NoSchedule-

# install rook

/dev/sda2