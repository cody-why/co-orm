/*
 * @Author: plucky
 * @Date: 2022-10-21 17:23:16
 * @LastEditTime: 2022-10-24 21:28:29
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
    // #[orm_rename = "users"] // rename table name
    struct User {
        // #[orm_pk] // default first field is primary key
        #[orm_seq] // sequence field, insert will ignore this field
        pub id: i64,
        
        #[orm_by] // generate query_by_field,update_by_field,delete_by_field
        #[orm_update] // generate method update_xxx. 
        //#[orm_rename = "name"] // rename field name
        pub name: Option<String> ,

        #[orm_ignore] // ignore field
        pub add: String,
        // #[sqlx(default)]
        // pub age: i32,
    }

    impl User {
        pub fn new(id: i64, name: Option<String>, add: String) -> Self { Self { id, name, add } }
    }

    #[cfg(feature = "mysql")]
    pub async fn get_pool() -> sqlx::Result<sqlx::MySqlPool> {
        sqlx::mysql::MySqlPoolOptions::new().connect("mysql://root:newpassword@192.168.1.199:3306/hello").await
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
        let u =User::query_by(&pool, "").await;
        println!("list {:?}",u);
    }

    #[tokio::test]
    async fn test_update(){
        let pool=get_pool().await.unwrap();

        let _u = User::new(1, Some("jack".into()), "sh".into());
        
        let r = _u.update(&pool).await.unwrap();
        dbg!(r);
        let r = _u.update_by(&pool,"where id=1").await.unwrap();
        dbg!(r);
        let r =  _u.update_name(&pool).await.unwrap();
        dbg!(r);
        
    }

    #[tokio::test]
    async fn test_insert() {
        let pool=get_pool().await.unwrap();
        let _u = User::new(0, Some("lusy".into()), "sz".into());
        let r =_u.insert(&pool).await.unwrap();
        println!("list: {:?}",r);
    }

    #[tokio::test]
    async fn test_delete() {
        let pool=get_pool().await.unwrap();
        
        let _u = User::new(4, Some("lusy".into()), "sz".into());
        let r = _u.delete(&pool).await.unwrap();
        println!("delete: {:?}",r);
        let r =User::delete_by(&pool, "where name='leo'").await.unwrap();
        println!("delete: {:?}",r);
        let r =User::delete_by_name(&pool, "lusy".into()).await.unwrap();
        println!("delete: {:?}",r);
    }

    
}