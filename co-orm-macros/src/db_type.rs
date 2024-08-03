/*
 * @Author: plucky
 * @Date: 2022-10-20 00:29:48
 * @LastEditTime: 2024-08-03 23:57:05
 */

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite", feature = "mssql")))]
pub mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream) {
        let pool = quote!(sqlx::MySqlPool);
        let query_result = quote!(sqlx::mysql::MySqlQueryResult);
        (pool, query_result)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::mysql::MySqlRow)
    }

    pub fn db_arguments_token() -> TokenStream {
        quote!(sqlx::mysql::MySqlArguments)
    }

    pub fn db_placeholder(_index: usize) -> String {
        "?".into()
    }
}

#[cfg(feature = "mysql")]
pub mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream) {
        let pool = quote!(sqlx::MySqlPool);
        let query_result = quote!(sqlx::mysql::MySqlQueryResult);
        (pool, query_result)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::mysql::MySqlRow)
    }

    pub fn db_arguments_token() -> TokenStream {
        quote!(sqlx::mysql::MySqlArguments)
    }

    pub fn db_placeholder(_index: usize) -> String {
        "?".into()
    }
}

#[cfg(feature = "postgres")]
pub mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream) {
        let pool = quote!(sqlx::PgPool);
        let query_result = quote!(sqlx::postgres::PgQueryResult);
        (pool, query_result)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::postgres::PgRow)
    }

    pub fn db_arguments_token() -> TokenStream {
        quote!(sqlx::postgres::PgArguments)
    }

    pub fn db_placeholder(index: usize) -> String {
        format!("${}", index)
    }
}

#[cfg(feature = "sqlite")]
pub mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream) {
        let pool = quote!(sqlx::SqlitePool);
        let query_result = quote!(sqlx::sqlite::SqliteQueryResult);
        (pool, query_result)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::sqlite::SqliteRow)
    }

    pub fn db_arguments_token() -> TokenStream {
        quote!(sqlx::sqlite::SqliteArguments<'_>)
    }

    pub fn db_placeholder(index: usize) -> String {
        format!("?{}", index)
    }
}

#[cfg(feature = "mssql")]
pub mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream) {
        let pool = quote!(sqlx::Pool<sqlx::Mssql>);
        let query_result = quote!(sqlx::mssql::MssqlQueryResult);
        (pool, query_result)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::mssql::MssqlRow)
    }

    pub fn db_arguments_token() -> TokenStream {
        quote!(sqlx::mssql::MssqlArguments)
    }

    pub fn db_placeholder(_index: usize) -> String {
        "?".into()
    }
}
