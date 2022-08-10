use crate::schema::users;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[allow(unused)]
#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub token: Option<String>,
}
