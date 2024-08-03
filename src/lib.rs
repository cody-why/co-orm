/*
 * @Author: plucky
 * @Date: 2022-10-21 16:53:21
 * @LastEditTime: 2024-08-03 23:52:00
 */

//! Derive macro for sqlx to implement Create, Read, Update, and Delete (CRUD) methods.
//! # Use
//! adding the following to your project's Cargo.toml:
//! ```toml
//! [dependencies]
//! co-orm = { virsion = "0.3", features = ["mysql"] }
//! sqlx = { version = "0.7", features = ["mysql"] }
//! ```
//! features: mysql, postgres, sqlite, mssql
//!
//! # Examples
//! ``` no_run,ignore
//! use co_orm::{Crud, args};
//!
//! #[derive(Debug, Crud, sqlx::FromRow)]
//! #[co_orm(rename = "users")] // rename table name
//! pub struct User {
//!     // #[co_orm(id)] // default first field is primary key
//!     #[co_orm(seq)] // sequence field, insert will ignore this field
//!     pub id: u64,
//!     #[co_orm(rename = "name")] // rename field name
//!     #[co_orm(by)] // generate query_by_field,update_by_field,delete_by_field
//!     pub name: String,
//!     #[co_orm(update)] // generate method update_xxx.
//!     pub password: String,
//!     #[co_orm(skip)] // ignore field
//!     #[sqlx(skip)]
//!     pub addr: Option<String>,
//!     // #[co_orm(skip_insert)] // insert will skip this field.
//!     // pub update_at: Option<NaiveDateTime>,
//! }
//!
//! // use crud
//! let u = User::get(&pool, 1).await;
//! println!("get {:?}", u);
//! let u = User::get_by(&pool, "where id=?", args!(1)).await;
//! println!("get_by {:?}", u);
//! ```

mod macros;

pub use co_orm_macros::Crud;
pub use co_orm_macros::FromRow;
