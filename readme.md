# Rust derive macro for sqlx to implement Create, Read, Update, and Delete (CRUD) methods.

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/co-orm">
    <img src="https://img.shields.io/crates/v/co-orm.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  
  <!-- Docs -->
  <a href="https://docs.rs/co-orm">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/co-orm">
    <img src="https://img.shields.io/crates/d/co-orm.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

## Use
 adding the following to your project's Cargo.toml:
 ```toml
[dependencies]
sqlx = { version = "0.6", features = ["mysql","runtime-tokio-native-tls"] }
co-orm = { virsion = "0.2.3", features = ["mysql"] }
# for #[tokio::test]
tokio = { version = "1", features = ["macros"] }

 ```
 
 * features: mysql, postgres, sqlite, mssql

## Examples
```rust
use co_orm::{Crud, FromRow};
#[derive(Debug, Crud, FromRow)]
// #[orm_rename = "users"] // rename table name
pub struct User {
    // #[orm_pk] // default first field is primary key
    #[orm_seq] // sequence field, insert will ignore this field
    pub id: i64,
    
    #[orm_by] // generate query_by_field,update_by_field,delete_by_field
    #[orm_update] // generate method update_xxx. 
    //#[orm_rename = "name"] // rename field name
    pub name: Option<String> ,

    #[orm_ignore] // ignore field
    pub add: String,
}

pub async fn get_pool() -> Result<MySqlPool> {
    MySqlPoolOptions::new()
        .connect("mysql://root:password@192.168.1.199:3306/hello").await
}

#[tokio::test]
async fn test_query() {
    let pool=get_pool().await.unwrap();
    let u = User::get(&pool, 1).await.unwrap();
    println!("get {:?}", u);
    let u = User::get_by(&pool, "where id=1").await.unwrap();
    println!("get {:?}", u);
    
}

```

``` rust
/// #[derive(Crud)]
/// generate get, get_by, query_by, update, delete, insert.
/// 
/// attributes:
/// 
/// #[orm_pk]
/// default first field is primary key or set.
/// 
/// `#[orm_seq]`
/// sequence field, auto increment. insert will ignore this field.
/// 
/// #[orm_rename= "name"]
/// rename table name or field name.
/// default table name by struct name to_table_case: UserDetail => user_details.
/// default field name by field name to_snake_case: UserDetail => user_detail.
/// 
/// #[orm_ignore]
/// ignore field.
/// 
/// #[orm_update]
/// generate method update_xxx. 
/// 
/// #[orm_by]
/// generate query_by_field,update_by_field,delete_by_field.
///
/// #[derive(FromRow)]
/// impl sqlx::FromRow trait.
/// 
/// if use `#[derive(FromRow)]` macro, must use `#[derive(Crud)]` macro.
/// 
/// if you don't want to use `#[derive(co_orm::FromRow)]` macro, 
/// you can use `#[derive(sqlx::FromRow)]` macro or impl `sqlx::FromRow` trait.
/// 
/// if using sqlx::FromRow, `#[orm_ignore]` add `#[sql::defult]` .


```
