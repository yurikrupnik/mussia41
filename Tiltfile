docker_build(
  "yurikrupnik/operators-orc",
  ".",
  dockerfile="./Dockerfile",
  build_args={"APP_NAME":"web_ntex"},
  target="final",
)
# operators_orc = 19MB
# web_ntex = 2.38MB
# web_actix = 20MB
# web_axum = 6.97MB

k8s_yaml(kustomize('manifests/base'))

# include('./apps/web/actix/Tiltfile')