
TOKEN=$(kubectl get secret $(kubectl get serviceaccount default -o jsonpath='{.secrets[0].name}') -o jsonpath='{.data.token}' | base64 --decode)

APISERVER=$(kubectl config view --minify -o jsonpath='{.clusters[0].cluster.server}')
curl -k -H "Authorization: Bearer $TOKEN" \
    -H "Accept: application/json" \
    "$APISERVER/api/v1/namespaces?labelSelector=app%3Dshit" | jq '.items[].metadata.name'
