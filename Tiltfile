docker_build(
  "yurikrupnik/operators-orc",
  ".",
  dockerfile="./rust.Dockerfile",
  build_args={"APP_NAME":"web_axum"},
  target="final",
)
# operators_orc = 19MB
# web_ntex = 2.38MB
# web_actix = 20MB
# web_axum = 19.2MB

k8s_yaml(kustomize('manifests/base'))

# include('./apps/web/actix/Tiltfile')
