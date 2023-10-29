/*
 * @Author: plucky
 * @Date: 2022-10-21 17:23:16
 * @LastEditTime: 2023-10-29 12:14:47
 * @Description: 
 */
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    #![allow(unused)]
    use co_orm::{Crud, FromRow};

    // #[derive(sqlx::FromRow)]
    #[derive(Debug, Crud, FromRow)]
    #[co_orm(rename = "users")] // rename table name
    struct User {
        // co_orm(id) // default first field is primary key
        #[co_orm(seq)] // sequence field, insert will ignore this field
        pub id: u64,
        
        #[co_orm(by)] // generate query_by_field,update_by_field,delete_by_field
        pub name: String,
        #[co_orm(update)] // generate method update_xxx. 
        pub password: String,
        #[co_orm(skip)] // ignore field
        pub addr: Option<String>,
        // #[co_orm(rename = "age")] // rename field name
        // #[sqlx(skip)]
        // pub age: i32,
    }

    impl User {
        pub fn new(id: u64, name: impl Into<String>, password: impl Into<String>) -> Self { 
            Self { id, name:name.into(), password: password.into(), addr: None } 
        }
    }

    #[cfg(feature = "mysql")]
    pub async fn get_pool() -> sqlx::Result<sqlx::MySqlPool> {
        sqlx::mysql::MySqlPoolOptions::new().connect("mysql://root:789789@192.168.1.199:3306/hello").await
    }

    #[cfg(feature = "postgres")]
    pub async fn get_pool() -> sqlx::Result<sqlx::PgPool> {
        sqlx::postgres::PgPoolOptions::new().connect("postgres://postgres:password@192.168.1.199:5432/postgres").await
    }

    #[tokio::test]
    async fn test_query() {
        let pool=get_pool().await.unwrap();
        
        let u = User::get(&pool, 1).await;
        println!("get {:?}", u);
        let u = User::get_by(&pool, "where id=1").await;
        println!("get_by {:?}", u);
        let u = User::query_by_name(&pool, "plucky".into()).await;
        println!("query_by_name {:?}", u);
        let u =User::query(&pool).await;
        println!("list {:?}",u);
    }

    #[tokio::test]
    async fn test_update(){
        let pool=get_pool().await.unwrap();

        let _u = User::new(2, "jack", "123456");
        
        let r = _u.update(&pool).await.unwrap();
        dbg!(r);
        let r = _u.update_by(&pool,"where id=2").await.unwrap();
        dbg!(r);
        let r =  _u.update_password(&pool).await.unwrap();
        dbg!(r);
        
    }

    #[tokio::test]
    async fn test_insert() {
        let pool=get_pool().await.unwrap();
        let _u = User::new(0, "lusy", "123456");
        let r =_u.insert(&pool).await.unwrap();
        println!("list: {:?}",r);
    }

    #[tokio::test]
    async fn test_delete() {
        let pool=get_pool().await.unwrap();
        
        let _u = User::new(10, "lusy", "123456");
        let r = _u.delete(&pool).await.unwrap();
        println!("delete: {:?}",r);
        let r =User::delete_by(&pool, "where name='leo'").await.unwrap();
        println!("delete: {:?}",r);
        let r =User::delete_by_name(&pool, "lusy".into()).await.unwrap();
        println!("delete: {:?}",r);
    }

    #[tokio::test]
    async fn test_insert_all() {
        let pool=get_pool().await.unwrap();
        let list = vec![User::new(0, "lusy3", "123456"),User::new(0, "lusy5", "123456")];
        let r =User::insert_all(&pool, list).await.unwrap();
        println!("list: {:?}",r);

    }

    
}