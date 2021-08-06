sudo apt update
sudo apt dist-upgrade
sudo reboot

# https://kubernetes.io/docs/reference/setup-tools/kubeadm/
# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/
# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/

create three servers

create network kubernetes-network
add servers

later: use load balancer, now: add dns to node-1 kube-apiserver.selfmade4u.de
https://github.com/kubernetes/kubeadm/blob/master/docs/ha-considerations.md#keepalived-and-haproxy

https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/

ssh root@kubernetes-node-x.selfmade4u.de

sudo modprobe br_netfilter
cat <<EOF | sudo tee /etc/modules-load.d/k8s.conf
br_netfilter
EOF

cat <<EOF | sudo tee /etc/sysctl.d/k8s.conf
net.bridge.bridge-nf-call-ip6tables = 1
net.bridge.bridge-nf-call-iptables = 1
EOF
sudo sysctl --system

# containerd

cat <<EOF | sudo tee /etc/modules-load.d/containerd.conf
overlay
br_netfilter
EOF

sudo modprobe overlay
sudo modprobe br_netfilter

# Setup required sysctl params, these persist across reboots.
cat <<EOF | sudo tee /etc/sysctl.d/99-kubernetes-cri.conf
net.bridge.bridge-nf-call-iptables  = 1
net.ipv4.ip_forward                 = 1
net.bridge.bridge-nf-call-ip6tables = 1
EOF

# Apply sysctl params without reboot
sudo sysctl --system

curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
sudo apt install -y containerd.io
sudo mkdir -p /etc/containerd
containerd config default | sudo tee /etc/containerd/config.toml
sudo systemctl restart containerd

[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc]
  ...
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc.options]
    SystemdCgroup = true

sudo systemctl restart containerd

# installing kubeadm, kubelet, kubectl

sudo curl -fsSLo /usr/share/keyrings/kubernetes-archive-keyring.gpg https://packages.cloud.google.com/apt/doc/apt-key.gpg
echo "deb [signed-by=/usr/share/keyrings/kubernetes-archive-keyring.gpg] https://apt.kubernetes.io/ kubernetes-xenial main" | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo apt-get update
sudo apt-get install -y kubelet kubeadm kubectl
sudo apt-mark hold kubelet kubeadm kubectl

# https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/#stacked-control-plane-and-etcd-nodes

# https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/
# https://pkg.go.dev/k8s.io/kubelet/config/v1beta1?utm_source=godoc#KubeletConfiguration

sudo apt install -y apparmor-utils
# https://github.com/rancher/k3os/issues/702

kubeadm init --config kubeadm-config.yaml --upload-certs --ignore-preflight-errors=NumCPU

# on failure:
kubeadm reset
iptables -F && iptables -t nat -F && iptables -t mangle -F && iptables -X
rm -R /etc/cni/net.d

export KUBECONFIG=/etc/kubernetes/admin.conf

kubectl apply -f https://raw.githubusercontent.com/coreos/flannel/master/Documentation/kube-flannel.yml

kubectl get pod -n kube-system -w

kubectl logs -n kube-system kube-flannel-ds-q6cvz


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



dig kube-apiserver.selfmade4u.de


# reboot a node:
kubectl drain kubernetes-node-1 --ignore-daemonsets --delete-emptydir-data
# rescue system

e2fsck -f /dev/sda1
resize2fs /dev/sda1 5G
The filesystem on /dev/sda1 is now 1310720 (4k) blocks long.
fdisk /dev/sda
d
1
n
1
<enter>
+1310720*4 K
<no>
p
q
reboot
# now we have some free space

kubectl uncordon kubernetes-node-1

# TODO do the same with the two other nodes



kubectl taint nodes kubernetes-node-1 node-role.kubernetes.io/master:NoSchedule-
kubectl taint nodes kubernetes-node-2 node-role.kubernetes.io/master:NoSchedule-
kubectl taint nodes kubernetes-node-3 node-role.kubernetes.io/master:NoSchedule-

# install rook
