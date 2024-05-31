use super::model::{Namespace, NewNamespace, UpdateNamespace};
use crate::AppState;
use k8s_openapi::api::core::v1::{Namespace as K8sNamespace, Pod};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams};
use ntex::web::{
    types::{Json, Path, State},
    HttpResponse,
};
use serde_json::json;

/// Get a `Namespace` by id
#[utoipa::path(
    get,
    path = "/api/namespace",
    tag = "Namespaces",
    responses(
    // (status = 200, description = "Service found", body = Vec<K8sNamespace>),
    (status = 404, description = "Service not found", body = HttpError),
    ),
)]
pub async fn list_namespaces(app_state: State<AppState>) -> HttpResponse {
    let client = app_state.client.clone();
    let namespaces: Api<K8sNamespace> = Api::all(client);
    let s = ListParams::default().limit(10).labels("shit=shit");
    match namespaces.list(&ListParams::default()).await {
        Ok(namespace_list) => {
            // let namespace_names: Vec<String> = namespace_list.iter()
            //     .filter_map(|ns| ns.metadata.name.clone())
            //     .collect();
            HttpResponse::Ok().json(&namespace_list.items)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error listing namespaces: {:?}", e))
        }
    }
}

/// Create a new `Namespace`
#[utoipa::path(
    post,
    path = "/api/service",
    tag = "Namespaces",
    request_body = NewNamespace,
    responses(
    (status = 201, description = "Service created", body = Service),
    ),
)]
pub async fn create_namespace(
    app_state: State<AppState>,
    body: Json<NewNamespace>,
) -> HttpResponse {
    let client = app_state.client.clone();
    let api: Api<K8sNamespace> = Api::all(client);
    let s = body.into_inner();
    // s.name
    let name = s.name.clone().unwrap();
    // let labels = s.labels.clone().unwrap();
    let pod = Pod {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            // annotations: Some(s.annotations.clone()),
            // labels: labels
            // name: Some(ns_spec.name.clone()),
            // labels: ns_spec.labels.clone(),
            ..Default::default()
        },
        ..Default::default()
    };
    let namespace = K8sNamespace {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            // annotations: Some(s.annotations.clone()),
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

pub async fn delete_namespace(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let client = app_state.client.clone();
    let api: Api<K8sNamespace> = Api::all(client);
    let id = id.into_inner();
    match api.delete(&id, &DeleteParams::default()).await {
        Ok(_) => HttpResponse::Ok().body(format!("Namespace '{}' deleted successfully", id)),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error deleting namespace: {:?}", e))
        }
    }
}

pub async fn update_namespace(
    app_state: State<AppState>,
    id: Path<String>,
    body: Json<UpdateNamespace>,
) -> HttpResponse {
    let client = app_state.client.clone();
    let api: Api<K8sNamespace> = Api::all(client);
    let id = id.into_inner();
    let patch = json!({
        "metadata": {
            "labels": {
                "app": "shit",
                "env": "docker"
            }
            // "labels": labels.unwrap_or_default(),
            // "annotations": annotations.unwrap_or_default()
        }
    });
    match api
        .patch_metadata(&id, &PatchParams::default(), &Patch::Merge(&patch))
        .await
    {
        Ok(namespace) => {
            println!("Namespace updated successfully {:?}", namespace);
            HttpResponse::Ok().body("all good")
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error updating namespace: {:?}", e))
        }
    }
}
