#![allow(unused)]
fn main() {}

use co_orm::{Crud, FromRow};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Crud, FromRow, Clone)]
#[co_orm(rename = "users")] // rename table name
pub struct User {
    // #[co_orm(id)] // default first field is primary key, for update and delete.
    #[co_orm(skip_insert)] // insert will ignore this field
    pub id: i64,
    #[co_orm(rename = "name")] // rename field name
    // #[sqlx(rename = "name")]
    pub name: String,
    #[co_orm(update)] // generate method update_xxx.
    pub password: String,
    pub age: Option<u32>,
    #[co_orm(skip)] // ignore field
    // #[sqlx(skip)]
    pub skip: Option<String>,
    // pub amount: Option<BigDecimal>, // not support sqlite
    #[co_orm(skip_insert)] // insert will ignore this field
    pub update_at: Option<NaiveDateTime>,
    pub status: Option<i32>,
}

impl User {
    pub fn new(id: i64, name: impl Into<String>, password: impl Into<String>, age: u32) -> Self {
        Self {
            id,
            name: name.into(),
            password: password.into(),
            age: Some(age),
            skip: None,
            update_at: None,
            status: None,
        }
    }
}
