# delete k3d cluster
k3d cluster delete $CLUSTER_NAME

# create k3d cluster 
k3d cluster create $CLUSTER_NAME -p "${CLUSTER_PORT}:80@loadbalancer"
k3d kubeconfig write gws

helm upgrade --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard --create-namespace --namespace kubernetes-dashboard

kubectl create sa kube-ds-admin -n kube-system
kubectl create clusterrolebinding kube-ds-admin-role-binding --clusterrole=admin --user=system:serviceaccount:kube-system:kube-ds-admin

# recreate the database
sqlx database reset --source ./api/migrations/ -y -f