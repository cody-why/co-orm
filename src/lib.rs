/*
* @Author: plucky
* @Date: 2022-10-21 16:53:21

*/

//! Derive macro for sqlx to implement Create, Read, Update, and Delete (CRUD) methods.
//! # Use
//! adding the following to your project's Cargo.toml:
//! ```toml
//! [dependencies]
//! co-orm = { virsion = "0.3", features = ["mysql"] }
//! sqlx = { version = "0.8", features = ["mysql"] }
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
//!     #[co_orm(skip_insert)] // insert will ignore this field
//!     pub id: i64,
//!     #[co_orm(rename = "name")] // rename field name
//!     pub name: String,
//!     #[co_orm(update)] // generate method update_xxx.
//!     pub password: String,
//!     #[co_orm(skip)] // ignore field
//!     #[sqlx(skip)]
//!     pub addr: Option<String>,
//!     #[co_orm(skip_insert)] // insert will skip this field.
//!     pub update_at: Option<NaiveDateTime>,
//! }
//!
//! // use crud
//! let u = User::get(&pool, 1).await;
//! println!("get {:?}", u);
//! let u = User::get_by(&pool, "where id=?", args!(1)).await;
//! println!("get_by {:?}", u);
//! let u = User::query_where(&pool, Where::new().eq("name", "jack")).await;
//! println!("query_where {:?}", u);
//! let u = User::query_page_where(&pool, Where::new().eq("name", "jack"), 1, 10).await;
//! println!("query_page_where {:?}", u);
//! let u = User::update_where(&pool, Where::new().eq("name", "jack")).await;
//! println!("update_where {:?}", u);
//! let u = User::delete_where(&pool, Where::new().eq("name", "jack")).await;
//! println!("delete_where {:?}", u);
//! ```

mod macros;

pub use co_orm_macros::Crud;
pub use co_orm_macros::FromRow;
mod filter;
pub use filter::Where;
