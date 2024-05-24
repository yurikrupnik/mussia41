use ntex::web::{self, HttpResponse, HttpRequest, HttpServer, App};
use k8s_openapi::api::core::v1::{Pod, PodSpec, Container, Namespace as K8sNamespace};
use kube::{Client, api::{Api, ListParams, PostParams}};
use crate::AppState;
use super::model::{Namespace, NewNamespace};
// use serde::{Serialize, Deserialize};
// use serde::{Serialize, Deserialize};
// // use kube::runtime::Reflector;
// // use ntex::http::body::Body;
// use ntex::web::types::{Json, Path};

            // let namespace_names: Vec<String> = namespace_list.iter().map(|ns| ns.metadata.name.clone().unwrap_or_default()).collect();
pub async fn list_namespaces(app_state: State<AppState>) -> HttpResponse {
    let client = app_state.client.clone();
    let namespaces: Api<K8sNamespace> = Api::all(client);
    match namespaces.list(&ListParams::default()).await {
        Ok(namespace_list) => {
            // let namespace_names: Vec<String> = namespace_list.iter()
            //     .filter_map(|ns| ns.metadata.name.clone())
            //     .collect();
            HttpResponse::Ok().json(&namespace_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error listing namespaces: {:?}", e)),
    }
}
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use ntex::web::types::{Json, State};

pub async fn create_namespace(app_state: State<AppState>, body: Json<NewNamespace>) -> HttpResponse {
    let client = app_state.client.clone();
    let namespaces: Api<K8sNamespace> = Api::all(client);
    let s = body.into_inner();
    // s.name
    let name = s.name.clone().unwrap();
    let labels = s.labels.clone().unwrap();
    let namespace = K8sNamespace {
        metadata: ObjectMeta {
            name: Some(name),
            // labels: labels
            // name: Some(ns_spec.name.clone()),
            // labels: ns_spec.labels.clone(),
            ..Default::default()
        },
        ..Default::default()
    };
    match namespaces.create(&PostParams::default(), &namespace).await {
        Ok(namespace_list) => {
            // let namespace_names: Vec<String> = namespace_list.iter().map(|ns| ns.metadata.name.clone().unwrap_or_default()).collect();
            // HttpResponse::Ok().body("ar");
            HttpResponse::Ok().json(&namespace_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error listing namespaces: {:?}", e)),
    }
}