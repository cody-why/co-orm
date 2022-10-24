/*
 * @Author: plucky
 * @Date: 2022-10-20 00:29:48
 * @LastEditTime: 2022-10-24 08:48:57
 * @Description: 
 */

use quote::{quote, __private::TokenStream};

/// return (pool, query_result)
pub fn db_pool_token()->(TokenStream, TokenStream){
    //Postgres, MySql, Sqlite, Mssql.
    // let mut db = "sqlx::MySql";
    let mut pool = quote!(sqlx::Pool<sqlx::MySql>);
    let mut query_result = quote!(sqlx::mysql::MySqlQueryResult);
    if cfg!(feature = "postgres") {
        // db = "sqlx::Postgres";
        pool = quote!(sqlx::Pool<sqlx::Postgres>);
        query_result = quote!(sqlx::postgres::PgQueryResult);
    }
    if cfg!(feature = "sqlite") {
        // db = "sqlx::Sqlite";
        pool = quote!(sqlx::Pool<sqlx::Sqlite>);
        query_result = quote!(sqlx::sqlite::SqliteQueryResult);
    }
    if cfg!(feature = "mssql") {
        // db = "sqlx::Mssql";
        pool = quote!(sqlx::Pool<sqlx::Mssql>);
        query_result = quote!(sqlx::mssql::MssqlQueryResult);
    }
    (pool, query_result)
}

/// dbRow for FromRow
pub fn db_row_token()->TokenStream{
    let mut row = quote!(sqlx::mysql::MySqlRow);
    if cfg!(feature = "postgres") {
        row = quote!(sqlx::postgres::PgRow);
    }
    if cfg!(feature = "sqlite") {
        row = quote!(sqlx::sqlite::SqliteRow);
    }
    if cfg!(feature = "mssql") {
        row = quote!(sqlx::mssql::MssqlRow);
    }
    row

}

/// sql format arg ? or $1
pub fn db_placeholder(index: usize)-> String{
   
    if cfg!(feature = "postgres") {
        return format!("${}", index);
    }
    if cfg!(feature = "sqlite") {
        return format!("?{}", index);
    }
    // if cfg!(feature = "mssql") {
    //     return "?";
    // }
    return "?".into();

}
