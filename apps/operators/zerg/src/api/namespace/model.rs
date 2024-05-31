// use mongodb::bson::oid::ObjectId;
use k8s_openapi::api::core::v1::{Container, Namespace as K8sNamespace, Pod, PodSpec};
use kube::api::{ObjectMeta, ResourceExt};
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Namespace struct
#[derive(
    Default, Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, DbResource, TS,
)]
#[schema(example = json!({"id": "6646396301dcad222bba63b3", "text": "Buy food", "completed": false}))]
pub struct Namespace {
    // #[platten(ObjectMeta)]
    // asd: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    // pub labels: Option<std::collections::BTreeMap<String, String>>,
}

/// UpdateNamespace used to update an existing `Namespace`
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[schema(example = json!({"text": "Buy food"}))]
pub struct NewNamespace {
    /// Todo text
    // #[validate(length(min = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace labels
    pub labels: Option<std::collections::BTreeMap<String, String>>,
}

use std::collections::HashMap;

// #[schema(example = json!({"text": "Buy food"}))]
/// UpdateNamespace used to update an existing `Namespace`
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct UpdateNamespace {
    /// Namespace labels
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    extra: HashMap<String, String>,
    // #[serde(flatten)]
    // extra: HashMap<String, Value>,
}
