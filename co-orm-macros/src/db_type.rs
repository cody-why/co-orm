/*
 * @Author: plucky
 * @Date: 2022-10-20 00:29:48
 * 
 */

#[cfg(not(any(feature = "postgres", feature = "sqlite", feature = "mssql")))]
pub(crate) mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream, TokenStream) {
        let pool = quote!(sqlx::MySqlPool);
        let query_result = quote!(sqlx::mysql::MySqlQueryResult);
        let arguments = quote!(sqlx::mysql::MySqlArguments);
        (pool, query_result, arguments)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::mysql::MySqlRow)
    }

    pub fn db_placeholder(_index: usize) -> String {
        "?".into()
    }
}

#[cfg(feature = "postgres")]
pub(crate) mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream, TokenStream) {
        let pool = quote!(sqlx::PgPool);
        let query_result = quote!(sqlx::postgres::PgQueryResult);
        let arguments = quote!(sqlx::postgres::PgArguments);
        (pool, query_result, arguments)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::postgres::PgRow)
    }

    pub fn db_placeholder(index: usize) -> String {
        format!("${}", index)
    }
}

#[cfg(feature = "sqlite")]
pub(crate) mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream, TokenStream) {
        let pool = quote!(sqlx::SqlitePool);
        let query_result = quote!(sqlx::sqlite::SqliteQueryResult);
        let arguments = quote!(sqlx::sqlite::SqliteArguments);
        (pool, query_result, arguments)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::sqlite::SqliteRow)
    }

    pub fn db_placeholder(index: usize) -> String {
        format!("?{}", index)
    }
}

#[cfg(feature = "mssql")]
pub(crate) mod db {
    use quote::{__private::TokenStream, quote};

    pub fn db_pool_token() -> (TokenStream, TokenStream, TokenStream) {
        let pool = quote!(sqlx::Pool<sqlx::Mssql>);
        let query_result = quote!(sqlx::mssql::MssqlQueryResult);
        let arguments = quote!(sqlx::mssql::MssqlArguments);
        (pool, query_result, arguments)
    }

    pub fn db_row_token() -> TokenStream {
        quote!(sqlx::mssql::MssqlRow)
    }

    pub fn db_placeholder(_index: usize) -> String {
        "?".into()
    }
}
