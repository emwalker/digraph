# Development setup

## NFS

The frontend pods mount a shared directory that hosts git repositories in ReadWriteMany mode.  Minikube does not support this type of persistent volume on its own, and you'll need to get NFS running on the host.  Once NFS is running on the host machine, you'll need to configure Minikube to mount an exported NFS volume.  Once these things have been done, the nfs-provisioner config under k8s/overlays/minikube/nfs-provisioner should handle the rest.

Start with these blog posts:
- https://www.tecmint.com/install-nfs-server-on-ubuntu/
- https://www.digitalocean.com/community/tutorials/how-to-set-up-an-nfs-mount-on-ubuntu-20-04
- https://mikebarkas.dev/2019/setup-nfs-for-minikube-persistent-storage/

## Kubernetes dependencies

Following are some dependencies included under k8s/.

### Postgres

```sh
$ helm template bitnami/postgresql --name-template default \
  --values k8s/base/default/postgresql-values.yaml > k8s/base/default/postgresql.yaml
```

### Redis

```sh
$ helm repo add bitnami https://charts.bitnami.com/bitnami
$ helm template bitnami/redis --name-template default \
  --values k8s/base/default/redis-values.yaml > k8s/base/default/redis.yaml
```
