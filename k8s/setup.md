# Setting up a cluster

## Kubernetes on DigitalOcean

Installing postgres:
```
$ curl https://raw.githubusercontent.com/kubernetes/helm/master/scripts/get > install-helm.sh
$ chmod +x install-helm.sh
$ ./install-helm.sh
$ kubectl -n kube-system create serviceaccount tiller
$ kubectl create clusterrolebinding tiller-cluster-rule --clusterrole=cluster-admin \
  --serviceaccount=kube-system:tiller
$ kubectl patch deploy --namespace kube-system tiller-deploy \
  -p '{"spec":{"template":{"spec":{"serviceAccount":"tiller"}}}}'
$ helm init --service-account tiller --upgrade
$ helm install --name postgres stable/postgresql
```

Loading data into postgres:
```
$ export PGPASSWORD=$(kubectl get secret --namespace default postgres-postgresql -o jsonpath="{.data.postgresql-password}" | base64 --decode)
# Create a database called "digraph_dev" from the sql prompt
$ psql --host 127.0.0.1 -U postgres -p 5433
$ psql --host 127.0.0.1 -U postgres -p 5433 -d digraph_dev < data.dump
```
