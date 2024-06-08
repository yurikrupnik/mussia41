// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use proc_macros::DbResource;

#[derive(DbResource, Debug, Deserialize, Serialize)]
pub struct User {
  username: String,
  email: String,
  password: String,
  // created_at: DateTime<Utc>,
  // updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct IdentifyableUser {
  pub id: usize,

  #[serde(flatten)]
  pub item: User,
  // created_at: DateTime<Utc>,
  // updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
  pub id: usize,

  #[serde(flatten)]
  pub item: Option<User>,
  // created_at: DateTime<Utc>,
  // updated_at: DateTime<Utc>,
}

fn ads() {
  let user = User {
    username: "user".to_string(),
    email: "  ".to_string(),
    password: "  ".to_string(),
  };
}
