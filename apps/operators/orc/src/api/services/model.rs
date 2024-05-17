use mongodb::bson::oid::ObjectId;
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;
use services::mongo::{
    query_param_processing::QueryParamProcessing, serialize::serialize_option_object_id,
};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, DbResource)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_option_object_id"
    )]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
}

/// NewProject is used to create a new `Service`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema, DbResource)]
pub struct NewService {
    /// New project name
    #[validate(length(min = 3))]
    pub name: String,
}

/// UpdateService is used to update a `Service`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
pub struct UpdateService {
    #[validate(length(min = 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, IntoParams)]
#[serde(deny_unknown_fields)]
pub struct ServiceListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
    // pub projections: Option<Vec<String>>, // Add projections field.
}

impl QueryParamProcessing for ServiceListQuery {
    fn get_limit(&self) -> Option<String> {
        self.limit.clone()
    }

    fn clear_limit(&mut self) {
        self.limit = None;
    }

    fn get_projection(&self) -> Option<String> {
        self.projection.clone()
    }

    fn clear_projection(&mut self) {
        self.projection = None;
    }

    fn into_inner(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|_| json!({}))
    }
}
