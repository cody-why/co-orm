/*
 * @Author: plucky
 * @Date: 2022-10-21 17:23:16
 * @LastEditTime: 2024-03-18 17:16:44
 * @Description: 
 */
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    #![allow(unused)]

    use co_orm::{Crud, sql_args, query, query_as};
    use sqlx::{Execute, types::{chrono::NaiveDateTime, BigDecimal}};

    #[derive(Debug, Crud, sqlx::FromRow)]
    #[co_orm(rename = "users")] // rename table name
    struct User {
        // #[co_orm(id)] // default first field is primary key
        #[co_orm(seq)] // sequence field, insert will ignore this field
        pub id: u64,
        #[co_orm(rename = "name")] // rename field name
        #[sqlx(rename = "name")]
        #[co_orm(by)] // generate query_by_field,update_by_field,delete_by_field
        pub name: String,
        #[co_orm(update)] // generate method update_xxx. 
        pub password: String,
        #[co_orm(skip)] // ignore field
        #[sqlx(skip)]
        pub addr: Option<String>,
        
        pub amount: Option<BigDecimal>,
        #[co_orm(skip_insert)] // insert will ignore this field
        pub update_at: Option<NaiveDateTime>,
       
        // #[sqlx(skip)]
        // pub age: u32,
    }

    impl User {
        pub fn new(id: u64, name: impl Into<String>, password: impl Into<String>) -> Self { 
            Self { id, name:name.into(), password: password.into(), addr: None,  amount: None , update_at: None}
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

        let u = User::get(&pool, 2).await;
        println!("get {:?}", u);
        println!("");
        let u = User::get_by(&pool, "where id>?", sql_args!(1)).await;
        println!("get_by {:?}", u);
        println!("");
        let u = User::query_by_name(&pool, "jack".into()).await;
        println!("query_by_name {:?}", u);
        println!("");
        let u =User::query(&pool).await;
        println!("query {:?}",u);
        println!("");
        let u = User::query_by(&pool, "where id=? or id =?", sql_args!(1, 2)).await;
        println!("query_by {:?}", u);
    }


    #[tokio::test]
    async fn test_page() {
        let pool=get_pool().await.unwrap();
        let r =User::query_page_by(&pool, "where id>?", sql_args!(1), 1, 3).await;

        // println!("count: {:?}",r);
        if let Ok((count, list)) = r {
            println!("count: {}, list: {:?}", count, list);
        }
        
    }

    
    #[tokio::test]
    async fn test_update(){
        let pool=get_pool().await.unwrap();

        let _u = User::new(2, "jack", "123456a");
        
        let r = _u.update(&pool).await;
        println!("update {:?}",r);

        let r = _u.update_by(&pool,format!("where id={}",100)).await;
        println!("update_by {:?}",r);
        let r =  _u.update_password(&pool).await;
        println!("update_password {:?}",r);
        
    }

    #[tokio::test]
    async fn test_insert() {
        let pool=get_pool().await.unwrap();
        let _u = User::new(0, "lusy", "123456");
        let r =_u.insert(&pool).await;
        println!("list: {:?}",r);
        
    }

    #[tokio::test]
    async fn test_delete() {
        let pool=get_pool().await.unwrap();
        
        let _u = User::new(10, "lusy", "123456");
        let r = _u.delete(&pool).await;
        println!("delete: {:?}",r);
        let r =User::delete_by(&pool, "where name=?", sql_args!("leo")).await;
        println!("delete: {:?}",r);
        let r =User::delete_by_name(&pool, "lusy".into()).await;
        println!("delete: {:?}",r);
    }

    #[tokio::test]
    async fn test_insert_all() {
        let pool=get_pool().await.unwrap();
        let list = vec![User::new(0, "lusy1", "123456"),User::new(0, "lusy2", "123456")];
        let r =User::insert_all(&pool, list).await;
        println!("list: {:?}",r);

    }

    #[tokio::test]
    async fn test_args() {
        let args= sql_args!(1, "plucky");
        let sql = "select * from users where id = ? and name = ?";
        let sql = sqlx::query_as_with::<_,User, _>(sql, args).sql();
        println!("sql {:?}", sql);

        // let mut qb = sqlx::QueryBuilder::<'_, sqlx::MySql>::new("select * from users ");
        // qb.push("where id=")
        //     .push_bind(1)
        //     .push(" or name=")
        //     .push_bind("plucky");
        // println!("qb {:?}", qb.sql());

    }
   
    
}