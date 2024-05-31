use super::model::{CPod, Pod};
use crate::AppState;
use k8s_openapi::api::core::v1::{Container, Pod as K8sPod, PodSpec};
use kube::{
    api::{Api, ListParams, PostParams},
    Client,
};
use ntex::web::{self, App, HttpRequest, HttpResponse, HttpServer};
// use serde::{Serialize, Deserialize};
// use serde::{Serialize, Deserialize};
// // use kube::runtime::Reflector;
// // use ntex::http::body::Body;
// use ntex::web::types::{Json, Path};

// let namespace_names: Vec<String> = namespace_list.iter().map(|ns| ns.metadata.name.clone().unwrap_or_default()).collect();
pub async fn list_pods(app_state: State<AppState>) -> HttpResponse {
    let client = app_state.client.clone();
    let api: Api<K8sPod> = Api::all(client);
    match api.list(&ListParams::default()).await {
        Ok(namespace_list) => {
            // let namespace_names: Vec<String> = namespace_list.iter()
            //     .filter_map(|ns| ns.metadata.name.clone())
            //     .collect();
            HttpResponse::Ok().json(&namespace_list)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error listing namespaces: {:?}", e))
        }
    }
}
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use ntex::web::types::{Json, State};

pub async fn create_pod(app_state: State<AppState>, body: Json<CPod>) -> HttpResponse {
    let client = app_state.client.clone();
    let api: Api<K8sPod> = Api::all(client);
    let s = body.into_inner();
    // s.name
    let name = s.name.clone().unwrap();
    // let labels = s.labels.clone().unwrap();
    let pod = K8sPod {
        spec: PodSpec {
            // let name = "ssdasd".to_string();

            // let mut container = Container {
            //     name: Some("my-container".to_string()),
            //     image: Some("nginx".to_string()),
            //     ..Default::default()
            // };
            // let mut pod_spec = ObjectMeta {
            //     // containers: vec![container],
            //     ..Default::default()
            // };
            // container
            ..Default::default()
        },
        metadata: ObjectMeta {
            name: Some(name),
            // labels: labels
            // name: Some(ns_spec.name.clone()),
            // labels: ns_spec.labels.clone(),
            ..Default::default()
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
