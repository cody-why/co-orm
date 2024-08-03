/*
 * @Date: 2024-08-03 05:14:25
 * @LastEditTime: 2024-08-03 12:13:49
 */
fn main() {}
#[cfg(test)]
mod test_orm {
    #![allow(unused)]
    use co_orm::{args, query, query_as, Crud};
    use sqlx::{types::chrono::NaiveDateTime, Execute, FromRow};

    // sqlx::FromRow
    #[derive(Debug, Clone, co_orm::FromRow)]
    struct User {
        pub id: i64,
        pub name: String,
        pub password: String,
        // #[sqlx(default)]
        pub addr: Option<String>,
    }

    // impl<'r> FromRow<'r, sqlx::mssql::MssqlRow> for User {
    //     fn from_row(row: &'r sqlx::mssql::MssqlRow) -> sqlx::Result<Self> {
    //         todo!()
    //     }
    // }

    impl User {
        #[cfg(feature = "mssql")]
        pub async fn get_by(
            pool: &sqlx::MssqlPool, where_sql: impl AsRef<str>, args: sqlx::mssql::MssqlArguments,
        ) -> sqlx::Result<Self> {
            let sql = format!("SELECT {} FROM {} {}", "`id`,`name`,`password`", "users", where_sql.as_ref());
            sqlx::query_as_with::<_, Self, _>(&sql, args).fetch_one(pool).await
        }
        #[cfg(feature = "postgres")]
        pub async fn get_by(
            pool: &sqlx::PgPool, where_sql: impl AsRef<str>, args: sqlx::postgres::PgArguments,
        ) -> sqlx::Result<Self> {
            let sql = format!("SELECT {} FROM {} {}", "`id`,`name`,`password`", "users", where_sql.as_ref());

            sqlx::query_as_with::<_, Self, _>(&sql, args).fetch_one(pool).await
        }

        #[cfg(feature = "postgres")]
        pub async fn query_page_by(
            pool: &sqlx::PgPool, where_sql: impl AsRef<str>, args: sqlx::postgres::PgArguments, page: i32, page_size: i32,
        ) -> sqlx::Result<(i64, Vec<Self>)> {
            let sql = format!("SELECT {} FROM {} {}", "`id`,`name`,`password`", "users", where_sql.as_ref());

            // let total = sqlx::query_scalar_with::<_, i64, _>(&format!("select count(*) from ({}) as c", sql), args.0)
            //     .fetch_one(pool)
            //     .await?;
            let count_sql = format!("select count(*) from ({}) as c", sql);
            let mut a = sqlx::query_scalar_with::<_, i64, _>(&count_sql, args);
            let arg1 = a.take_arguments().unwrap_or_default().unwrap_or_default();
            let total = a.fetch_one(pool).await?;

            let sql = format!("{} LIMIT {} OFFSET {}", sql, page_size, page_size * (page - 1));
            sqlx::query_as_with::<_, Self, _>(&sql, arg1)
                .fetch_all(pool)
                .await
                .map(|list| (total, list))
        }

        #[cfg(feature = "sqlite")]
        pub async fn get_by(
            pool: &sqlx::SqlitePool, where_sql: impl AsRef<str>, args: sqlx::sqlite::SqliteArguments<'_>,
        ) -> sqlx::Result<Self> {
            let sql = format!("SELECT {} FROM {} {}", "`id`,`name`,`password`", "users", where_sql.as_ref());

            sqlx::query_as_with::<_, Self, _>(&sql, args).fetch_one(pool).await
        }
    }
}
