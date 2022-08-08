# Kubernetes dependencies

## Postgres

```sh
$ helm template bitnami/postgresql --name-template default \
  --values k8s/base/default/postgresql-values.yaml > k8s/base/default/postgresql.yaml
```

## Redis

```sh
$ helm template bitnami/redis --name-template default \
  --values k8s/base/default/redis-values.yaml > k8s/base/default/redis.yaml
```
