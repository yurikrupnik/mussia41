docker_build(
  "yurikrupnik/web-actix",
  ".",
  dockerfile="./Dockerfile",
  build_args={"APP_NAME":"web_actix"},
  target="rust",
)

k8s_yaml(kustomize('manifests/base'))

# include('./apps/web/actix/Tiltfile')