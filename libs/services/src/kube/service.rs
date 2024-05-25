use k8s_openapi::api::core::v1::{
    Container, Namespace as K8sNamespace, NamespaceSpec, Pod, PodSpec,
};
use kube::{
    api::{Api, ListParams, PostParams},
    Client,
};
use ntex::web::types::{Json, State};
use ntex::web::HttpResponse;
pub async fn create_namespace<T>(client: Client, body: T) -> HttpResponse {
    // let client = app_state.client.clone();
    let api: Api<K8sNamespace> = Api::all(client);
    // let s = body;
    // s.name
    // let name = body.name.clone().unwrap();
    // let labels = s.labels.clone().unwrap();
    let namespace = K8sNamespace {
        // metadata: ObjectMeta {
        // name: Some(name),
        // labels: labels
        // name: Some(ns_spec.name.clone()),
        // labels: ns_spec.labels.clone(),
        metadata: {
            let mut metadata = kube::api::ObjectMeta::default();
            metadata.name = Some("aris".to_string());
            metadata
        },
        spec: {
            let mut spec = NamespaceSpec::default();
            spec.finalizers = Some(vec!["kubernetes".to_string()]);
            Some(spec)
        },
        ..Default::default()
    };
    match api.create(&PostParams::default(), &namespace).await {
        Ok(namespace_list) => {
            // let namespace_names: Vec<String> = namespace_list.iter().map(|ns| ns.metadata.name.clone().unwrap_or_default()).collect();
            // HttpResponse::Ok().body("ar");
            HttpResponse::Ok().json(&namespace_list)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error listing namespaces: {:?}", e))
        }
    }
}
