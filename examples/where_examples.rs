mod pool;

#[tokio::main]
async fn main() {}

mod test_orm {
    #![allow(unused)]

    use crate::pool::get_pool;
    use co_orm::{args, page_args, query, query_as, Crud, Where};
    use sqlx::{
        types::{chrono::NaiveDateTime, BigDecimal},
        Execute,
    };

    #[derive(Debug, Crud, sqlx::FromRow)]
    #[co_orm(rename = "users")] // rename table name
    struct User {
        // #[co_orm(id)] // default first field is primary key
        #[co_orm(skip_insert)] // insert will ignore this field
        pub id: i64,
        #[co_orm(rename = "name")] // rename field name
        #[sqlx(rename = "name")]
        pub name: String,
        #[co_orm(update)] // generate method update_xxx.
        pub password: String,
        #[co_orm(skip)] // ignore field
        #[sqlx(skip)]
        pub addr: Option<String>,
        // pub amount: Option<BigDecimal>, // not support sqlite
        #[co_orm(skip_insert)] // insert will ignore this field
        pub update_at: Option<NaiveDateTime>,
        pub age: Option<u32>,
        pub status: Option<i32>,
    }

    impl User {
        pub fn new(id: i64, name: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                id,
                name: name.into(),
                password: password.into(),
                addr: None,
                // amount: None,
                update_at: None,
                age: None,
                status: None,
            }
        }
    }

    /// 基础 Where 方法使用示例
    #[tokio::test]
    pub async fn test_where_basic() {
        let pool = get_pool().await.unwrap();

        // 1. 基本比较操作
        let users =
            User::query_where(&pool, Where::new().eq("name", "jack").and().eq("age", 18)).await;
        println!("基本比较查询: {:?}", users);

        // 2. 使用 like 进行模糊查询
        let users = User::query_where(
            &pool,
            Where::new().like("name", "%jack%").and().like("password", "%123456%"),
        )
        .await;
        println!("模糊查询: {:?}", users);

        // 3. 使用 between 进行范围查询
        let users =
            User::query_where(&pool, Where::new().between("age", 18, 65).and().eq("status", 1))
                .await;
        println!("范围查询: {:?}", users);

        // 4. 使用 in 进行集合查询
        let users = User::query_where(&pool, Where::new().r#in("age", vec![18, 20])).await;
        println!("集合查询: {:?}", users);

        // 5. 使用 is_null 和 is_not_null
        let users =
            User::query_where(&pool, Where::new().is_not_null("name").and().is_null("age")).await;
        println!("NULL 值查询: {:?}", users);
    }

    /// 复杂 Where 条件组合示例
    #[tokio::test]
    pub async fn test_where_complex() {
        let pool = get_pool().await.unwrap();

        // 1. 复杂逻辑组合：年龄大于18且状态为active，或者邮箱包含特定域名
        let users = User::query_where(
            &pool,
            Where::new()
                .gt("age", 18)
                .and()
                .eq("status", 1)
                .or()
                .like("password", "%123456%"),
        )
        .await;
        println!("复杂逻辑组合: {:?}", users);

        // 2. 使用 and_group 和 or_group 进行分组
        let users = User::query_where(
            &pool,
            Where::new()
                .eq("status", 1)
                .and_group(|w| w.gt("age", 18).and().lt("age", 65))
                .or_group(|w| w.eq("name", "admin").or().eq("name", "root")),
        )
        .await;
        println!("分组条件查询: {:?}", users);

        // 3. 多字段组合查询
        let users = User::query_where(
            &pool,
            Where::new()
                .eq("status", 1)
                .and()
                .between("age", 18, 65)
                .and()
                .like("password", "%123456%")
                .and()
                .ne("name", "jerry"),
        )
        .await;
        println!("多字段组合查询: {:?}", users);
    }

    /// 使用 Where 进行分页查询示例
    #[tokio::test]
    pub async fn test_where_pagination() {
        let pool = get_pool().await.unwrap();

        // 1. 基础分页查询
        let (count, users) = User::query_page_where(
            &pool,
            Where::new().eq("status", 1), // 查询与计数条件
            1,                            // 页码
            10,                           // 每页大小
        )
        .await
        .unwrap();
        println!("分页查询 - 总数: {}, 用户: {:?}", count, users);

        // 2. 复杂条件分页查询
        let (count, users) = User::query_page_where(
            &pool,
            Where::new().eq("status", 1).and().ge("age", 18).and().like("name", "%jack%"),
            2,
            5,
        )
        .await
        .unwrap();
        println!("复杂分页查询 - 总数: {}, 用户: {:?}", count, users);
    }

    /// 使用 Where 进行更新和删除操作示例
    #[tokio::test]
    pub async fn test_where_crud() {
        let pool = get_pool().await.unwrap();

        // 1. 使用 Where 进行条件更新
        let mut user = User::new(0, "test_user", "new_password");
        user.age = Some(25);

        let result = user
            .update_where(&pool, Where::new().eq("name", "jack").and().eq("status", 1))
            .await;
        println!("条件更新结果: {:?}", result);

        // 2. 使用 Where 进行条件删除
        let result =
            User::delete_where(&pool, Where::new().eq("status", 1).and().lt("age", 18)).await;
        println!("条件删除结果: {:?}", result);

        // 3. 使用 Where 进行单条记录查询
        let user =
            User::get_where(&pool, Where::new().eq("name", "jack").and().eq("status", 1)).await;
        println!("条件查询单条记录: {:?}", user);
    }

    /// 高级 Where 用法示例
    #[tokio::test]
    pub async fn test_where_advanced() {
        let pool = get_pool().await.unwrap();

        // 1. 动态构建查询条件
        let mut where_builder = Where::new();

        // 根据条件动态添加
        where_builder = where_builder.eq("status", 1);

        if true {
            // 模拟某个条件
            where_builder = where_builder.and().ge("age", 18);
        }

        if true {
            // 模拟另一个条件
            where_builder = where_builder.and().like("password", "%123456%");
        }

        let users = User::query_where(&pool, where_builder).await;
        println!("动态构建查询: {:?}", users);

        // 2. 使用 raw 方法添加自定义 SQL 片段
        let users = User::query_where(
            &pool,
            Where::new()
                .eq("status", 1)
                .and()
                .raw("LENGTH(name) > 3") // 自定义 SQL 片段
                .and()
                .between("age", 18, 65),
        )
        .await;
        println!("自定义 SQL 片段查询: {:?}", users);

        // 3. 复杂嵌套分组查询
        let users = User::query_where(
            &pool,
            Where::new()
                .eq("status", 1)
                .and_group(|w| {
                    w.gt("age", 18).and().lt("age", 65).and_group(|w2| {
                        w2.like("password", "%123456%").or().like("password", "%123456%")
                    })
                })
                .or_group(|w| w.eq("name", "admin").and().ge("age", 21)),
        )
        .await;
        println!("复杂嵌套分组查询: {:?}", users);
    }

    /// 性能优化示例：使用 Where 构建器避免字符串拼接
    #[tokio::test]
    pub async fn test_where_performance() {
        let pool = get_pool().await.unwrap();

        // 1. 使用 Where 构建器（推荐）
        let users = User::query_where(
            &pool,
            Where::new()
                .eq("status", 1)
                .and()
                .ge("age", 18)
                .and()
                .like("password", "%123456%"),
        )
        .await;
        println!("Where 构建器查询: {:?}", users);

        // 2. 传统拼接方式（不推荐）
        let users = User::query_by(&pool, "where status=? and age>=?", args!(1, 18)).await;
        println!("字符串拼接查询: {:?}", users);

        // Where 构建器的优势：
        // - 类型安全
        // - 自动参数绑定
        // - 防止 SQL 注入
        // - 更好的可读性和维护性
    }
}
