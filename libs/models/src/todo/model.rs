use mongodb::bson::oid::ObjectId;
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;
use services::mongo::query_param_processing::QueryParamProcessing;
use services::mongo::serialize::serialize_option_object_id;
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

// #[derive(DbResource, ToSchema, Default, Serialize, Validate)]
#[derive(Default, Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, DbResource, TS)]
#[schema(example = json!({"id": "6646396301dcad222bba63b3", "text": "Buy food", "completed": false}))]
pub struct Todo {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_option_object_id"
    )]
    #[ts(type = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<bool>,
}

/// NewTodo is used to create a new `Todo`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
#[schema(example = json!({"text": "Buy food"}))]
pub struct NewTodo {
    #[validate(length(min = 2))]
    text: String,
    #[serde(default)]
    completed: bool,
}

/// UpdateTodo is used to update a `Todo`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
#[schema(example = json!({"completed": true}))]
pub struct Update {
    #[validate(length(min = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<bool>,
}
#[derive(Clone, Deserialize, Serialize, Debug, IntoParams)]
#[serde(deny_unknown_fields)]
pub struct TodoListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, IntoParams)]
#[serde(deny_unknown_fields)]
pub struct TodoItemQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

impl QueryParamProcessing for TodoListQuery {
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
