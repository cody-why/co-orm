# Implement Create, Read, Update, and Delete (CRUD) methods for sqlx.

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/co-orm">
    <img src="https://img.shields.io/crates/v/co-orm.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  
  <!-- Docs -->
  <a href="https://docs.rs/crate/co-orm">
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
 add the following to your project's Cargo.toml:
 ```toml
[dependencies]
co-orm = { virsion = "0.3", features = ["mysql"] }
sqlx = { version = "0.7", features = ["mysql","runtime-tokio-native-tls"] }


 ```
 
 * features: mysql, postgres, sqlite, mssql

## Examples
```rust
#[derive(Debug, Crud, sqlx::FromRow)]
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
    #[sqlx(skip)]
    pub addr: Option<String>,
    // 
    // pub age: i32,
    // #[co_orm(skip_insert)] // insert will ignore this field
}


pub async fn get_pool() -> Result<MySqlPool> {
    MySqlPoolOptions::new()
        .connect("mysql://root:password@192.168.1.199:3306/hello").await
}

#[tokio::test]
async fn test_query() {
    let pool=get_pool().await.unwrap();
    let u = User::get(&pool, 1).await;
    println!("get {:?}", u);
    let u = User::get_by(&pool, "where id=?", args!(1)).await;
    println!("get_by {:?}", u);
    let u = User::query_by_name(&pool, "plucky".into()).await;
    println!("query_by_name {:?}", u);
    let u =User::query(&pool).await;
    println!("list {:?}",u);

    // u.update(&pool).await;
    // u.insert(&pool).await;
    // u.delete(&pool).await

    // let list = vec![User::new(0, "lusy3", "123456"),User::new(0, "lusy5", "123456")];
    // let r =User::insert_all(&pool, list).await;
    // println!("list: {:?}",r);
    
    
}

```


## `#[derive(Crud)]`

generate method: get, get_by, query, query_by, update, delete, insert, insert_all, query_page_by

### attributes:

`#[co_orm(id)]`

default first field is primary key or set.

`#[co_orm(seq)]`

sequence field, auto increment. insert will skip this field.

`#[co_orm(skip_insert)]`

insert will skip this field.

`#[co_orm(rename="name")]`

rename table name or field name. 
default table name by struct name to_table_case: UserDetail => user_detail. 
default field name by field name to_snake_case: UserDetail => user_detail. 

`#[co_orm(skip)]`

ignore field. using sqlx::FromRow, skip need `#[co_orm(skip)]` and `#[sqlx(skip)]`

`#[co_orm(update)]`

generate method update_xxx. 

`#[co_orm(by)]`

generate qet_by_field, query_by_field, update_by_field, delete_by_field.

`#[co_orm(skip_insert)]`
insert will skip this field.




## macro_export


`args`
``` rust
 let args = args!(&name, age);
```

`query`
```rust
query!("insert into users (name, password) values (?,?)", name, password).execute(&pool).await
```
 
`query_as`
```rust
query_as!(User, "select * from users where name = ?", name).fetch_one(&pool).await
```