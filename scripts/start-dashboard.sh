# create login token
echo "Dashboard token: $(kubectl create token kube-ds-admin -n kube-system)"
# forward dashboard
echo -e "\n\nStart Dashboard:\n"
kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443