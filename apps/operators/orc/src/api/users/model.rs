use chrono::NaiveDateTime;
use mongodb::bson::oid::ObjectId;
use services::mongo::{
    query_param_processing::QueryParamProcessing, serialize::serialize_object_id,
};
// use redis::{FromRedisValue, ToRedisArgs};
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRedis {
    pub email: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
}

// #[derive(Debug, Deserialize, Serialize, Validate, ToSchema, DbResource)]
#[derive(Clone, ToSchema, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, DbResource)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename(deserialize = "_id"), serialize_with = "serialize_object_id")]
    pub id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    // pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

/// NewUser is used to create a new user
#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    /// Username is used to create a new user
    #[validate(length(min = 2))]
    #[schema(default = "john")]
    pub username: String,
    /// Email is used to create a new user
    #[validate(email)]
    #[schema(default = "john@gmail.com")]
    pub email: String,
    /// Password is used to create a new user
    #[schema(default = "12345")]
    #[validate(length(min = 5))]
    pub password: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub image: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateUser {
    // #[schema(default = "John Doe")]
    pub full_name: Option<String>,
    // #[schema(default = "I work at ...")]
    pub bio: Option<String>,
    #[validate(url)]
    // #[schema(default = "https://static.productionready.io/images/smiley-cyrus.jpg")]
    pub image: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, IntoParams)]
#[serde(deny_unknown_fields)]
pub struct UserListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
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

impl QueryParamProcessing for UserListQuery {
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
