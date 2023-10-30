# Rust derive macro implement Create, Read, Update, and Delete (CRUD) methods base on sqlx.

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
co-orm = { virsion = "0.2", features = ["mysql"] }
sqlx = { version = "0.7", features = ["mysql","runtime-tokio-native-tls"] }


 ```
 
 * features: mysql, postgres, sqlite, mssql

## Examples
```rust
// #[derive(sqlx::FromRow)]
#[derive(Debug, Crud, FromRow)]
#[co_orm(rename = "users")] // rename table name
struct User {
    // co_orm(id) // default first field is primary key
    #[co_orm(seq)] // sequence field, insert will ignore this field
    pub id: u64,
    [co_orm(rename = "name")] // rename field name
    #[co_orm(by)] // generate query_by_field,update_by_field,delete_by_field
    pub name: String,
    #[co_orm(update)] // generate method update_xxx. 
    pub password: String,
    #[co_orm(skip)] // ignore field
    pub addr: Option<String>,
    // #[sqlx(skip)]
    // pub age: i32,
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
### #[derive(Crud)]
```
`#[derive(FromRow)]`
generate method: get, get_by, query, query_by, update, delete, insert, insert_all.

attributes:

`#[co_orm(id)]`
default first field is primary key or set.

`#[co_orm(seq)]`
sequence field, auto increment. insert will skip this field.

`#[co_orm(rename="name")]`
rename table name or field name. 
default table name by struct name to_table_case: UserDetail => user_detail. 
default field name by field name to_snake_case: UserDetail => user_detail. 

`#[co_orm(skip)]`
ignore field.

`#[co_orm(update)]`
generate method update_xxx. 

`#[co_orm(by)]`
generate qet_by_field, query_by_field, update_by_field, delete_by_field.
```
### #[derive(FromRow)]
```
`#[derive(FromRow)]`
impl sqlx::FromRow trait.

or use `#[derive(sqlx::FromRow)]` macro or impl `sqlx::FromRow` trait.

if using sqlx::FromRow, if have `#[co_orm(skip)]` add `#[sqlx(skip)]` .

```


