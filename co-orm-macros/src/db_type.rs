/*
 * @Author: plucky
 * @Date: 2022-10-20 00:29:48
 * @LastEditTime: 2023-10-31 18:19:23
 * @Description: 
 */

 use quote::{quote, __private::TokenStream};

 /// return (pool, query_result)
 pub fn db_pool_token()->(TokenStream, TokenStream){
     //Postgres, MySql, Sqlite, Mssql.
     #[cfg(feature = "mysql")] {
         let pool = quote!(sqlx::Pool<sqlx::MySql>);
         let query_result = quote!(sqlx::mysql::MySqlQueryResult);
         (pool, query_result)
     }
 
     #[cfg(feature = "postgres")] {
         // db = "sqlx::Postgres";
         let pool = quote!(sqlx::Pool<sqlx::Postgres>);
         let query_result = quote!(sqlx::postgres::PgQueryResult);
         (pool, query_result)
     }
     #[cfg(feature = "sqlite")]{
         // db = "sqlx::Sqlite";
         let pool = quote!(sqlx::Pool<sqlx::Sqlite>);
         let query_result = quote!(sqlx::sqlite::SqliteQueryResult);
         (pool, query_result)
     }
     #[cfg(feature = "mssql")]{
         // db = "sqlx::Mssql";
         let pool = quote!(sqlx::Pool<sqlx::Mssql>);
         let query_result = quote!(sqlx::mssql::MssqlQueryResult);
         (pool, query_result)
     }
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
     "?".into()
 
 }
 
 
 pub fn db_arguments_token()->TokenStream{
     let mut row = quote!(sqlx::mysql::MySqlArguments);
     if cfg!(feature = "postgres") {
         row = quote!(sqlx::postgres::PgArguments);
     }
     if cfg!(feature = "sqlite") {
         row = quote!(sqlx::sqlite::SqliteArguments);
     }
     
     row
 
 }