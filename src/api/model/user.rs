use serde::{Deserialize, Serialize};
use wither::{
    bson::{doc, oid::ObjectId},
    Model,
};

#[derive(Debug, Serialize, Deserialize, Model)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,
}
