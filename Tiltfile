docker_build(
  "yurikrupnik/operators-orc",
  ".",
  dockerfile="./Dockerfile",
  build_args={"APP_NAME":"operators_orc"},
  target="final",
)

k8s_yaml(kustomize('manifests/base'))

# include('./apps/web/actix/Tiltfile')