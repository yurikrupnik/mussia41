use mongodb::bson::oid::ObjectId;
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;


/// Namespace struct
#[derive(Default, Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, DbResource, TS)]
#[schema(example = json!({"id": "6646396301dcad222bba63b3", "text": "Buy food", "completed": false}))]
pub struct Namespace {
    /// Todo text
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    pub labels: Option<std::collections::BTreeMap<String, String>>
}

/// NewNamespace is used to create a new `Namespace`
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[schema(example = json!({"text": "Buy food"}))]
pub struct NewNamespace {
    /// Todo text
    // #[validate(length(min = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace labels
    pub labels: Option<std::collections::BTreeMap<String, String>>
}