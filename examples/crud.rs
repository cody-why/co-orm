/*
* @Author: plucky
* @Date: 2022-10-21 17:23:16
*/

mod pool;
mod user;

#[tokio::main]
async fn main() {}

mod test_orm {
    #![allow(unused)]

    use crate::{pool::get_pool, user::User};
    use co_orm::{args, page_args, query, query_as, Where};
    use sqlx::Execute;

    #[tokio::test]
    pub async fn test_query() {
        let pool = get_pool().await.unwrap();

        let u = User::get(&pool, 2).await;
        println!("get {:?}", u);
        println!("");
        let u = User::get_by(&pool, "where name=?", args!("jack")).await;
        println!("get_by {:?}", u);
        println!("");

        let u = User::query(&pool).await;
        println!("query {:?}", u);
        println!("");
        let u = User::query_by(&pool, "where id=? or id =?", args!(1, 2)).await;
        println!("query_by {:?}", u);
    }

    #[tokio::test]
    async fn test_page() {
        let pool = get_pool().await.unwrap();
        let r = User::query_page_by(&pool, "where id>?", page_args!(1), 1, 3).await;

        // println!("count: {:?}",r);
        if let Ok((count, list)) = r {
            println!("count: {}, list: {:?}", count, list);
        }
    }

    #[tokio::test]
    async fn test_update() {
        let pool = get_pool().await.unwrap();

        let _u = User::new(2, "jack", "123456a", 18);

        let r = _u.update(&pool).await;
        println!("update {:?}", r);

        let r = _u.update_by(&pool, "where id = 2").await;
        println!("update_by {:?}", r);
    }

    #[tokio::test]
    async fn test_insert() {
        let pool = get_pool().await.unwrap();
        let _u = User::new(0, "lusy", "123456", 18);
        let r = _u.insert(&pool).await;
        println!("list: {:?}", r);
    }

    #[tokio::test]
    async fn test_delete() {
        let pool = get_pool().await.unwrap();

        let _u = User::new(10, "lusy", "123456", 18);
        let r = _u.delete(&pool).await;
        println!("delete: {:?}", r);
        let r = User::delete_by(&pool, "where name=?", args!("leo")).await;
        println!("delete: {:?}", r);
        // let r = User::delete_by_name(&pool, "lusy".into()).await;
        println!("delete: {:?}", r);
    }

    #[tokio::test]
    async fn test_insert_all() {
        let pool = get_pool().await.unwrap();
        let list = vec![
            User::new(0, "lusy1", "123456", 18),
            User::new(0, "lusy2", "123456", 18),
        ];
        let r = User::insert_all(&pool, list).await;
        println!("list: {:?}", r);
    }

    #[tokio::test]
    async fn test_args() {
        let args = args!(1, "plucky");
        let sql = "select * from users where id = ? and name = ?";
        // let arg2 = args.clone();
        let sql = sqlx::query_as_with::<_, User, _>(sql, args).sql();
        println!("sql {:?}", sql);

        // let mut qb = sqlx::QueryBuilder::<'_, sqlx::MySql>::new("select * from users ");
        // qb.push("where id=")
        //     .push_bind(1)
        //     .push(" or name=")
        //     .push_bind("plucky");
        // println!("qb {:?}", qb.sql());
    }
}
