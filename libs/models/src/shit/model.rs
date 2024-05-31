use mongodb::bson::{doc, oid::ObjectId};
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;
use services::mongo::{
    query_param_processing::QueryParamProcessing, serialize::serialize_option_object_id,
};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Shit struct
#[derive(Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, DbResource, TS)]
#[schema(example = json!({"id": "6646396301dcad222bba63b3", "text": "Buy food", "completed": false}))]
pub struct Shit {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_option_object_id"
    )]
    #[ts(type = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Title of the shit
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

/// NewShit is used to create a new `Shit`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema, TS)]
#[schema(example = json!({"text": "Buy drinks"}))]
pub struct NewShit {
    #[validate(length(min = 2))]
    text: String,
    // #[validate(range(min = 2, max = 2000))]
    // pages: u16,
}

/// UpdateShit is used to update a `Shit`
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema, TS)]
#[schema(example = json!({"completed": true}))]
pub struct UpdateShit {
    /// Title of the selected shit
    #[validate(length(min = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    // #[validate(range(min = 2, max = 2000))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pages: Option<u16>,
}
#[derive(Clone, Deserialize, Serialize, Debug, IntoParams, TS)]
#[serde(deny_unknown_fields)]
pub struct ShitListQuery {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pages: Option<u16>,
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

#[derive(Clone, Deserialize, Serialize, Debug, IntoParams, TS)]
#[serde(deny_unknown_fields)]
pub struct ShitItemQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

impl QueryParamProcessing for ShitListQuery {
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
