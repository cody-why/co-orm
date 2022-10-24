/*
 * @Author: plucky
 * @Date: 2022-10-22 18:08:45
 * @LastEditTime: 2022-10-24 10:35:21
 * @Description: 
 */

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};
use crate::db_type::*;
use crate::impl_by_field::*;
use crate::util::*;



///  generate_crud
pub fn generate_crud(input: DeriveInput) -> TokenStream {
    let table_name = get_table_name(&input);
    // println!("table_name: {}", table_name);

    let struct_name = &input.ident;

    let fields1 = match &input.data {
        Data::Struct(DataStruct {fields: Fields::Named(fields),..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    // let fields_all = fields.iter().collect::<Vec<_>>();

    // filter ignore fields
    let fields = fields1.iter().filter(|f| !is_ignore(f)).collect::<Vec<_>>();

    // insert skip seq field
    let fields_insert = fields.iter().filter(|f| !is_seq(f)).collect::<Vec<_>>();
   
    let field_name_insert = fields_insert.iter().filter(|f| !is_seq(f)).map(|field| {
        &field.ident
    }).collect::<Vec<_>>();

    // inset (a,b,c) values (?,?,?)
    let columns = fields_insert.iter().map(|field| {
        get_field_name(field)
    }).collect::<Vec<_>>().join(",");
    
    let values = question_marks(fields_insert.len());
    
    // let fields_list = quote! {
    //     #(#column_name),*
    // };

    // let columns = format!("{}", fields_list)
    //     .replace("\"", "");
        // .replace(",\n", ",");
    
    // upsert use all fields except ignore
    let field_name_all = fields.iter().map(|field| {
        &field.ident
    }).collect::<Vec<_>>();
    let columns_all = fields.iter().map(|field| {
        get_field_name(field)
    }).collect::<Vec<_>>().join(",");
    let values_all = question_marks(fields.len());

    // find `orm_pk` or default the first field as the "id" column
    let id_field = fields.iter()
        .find(|f| is_id(f))
        .unwrap_or_else(||fields.iter().next().unwrap());
    let id_column = id_field.ident.as_ref().unwrap();
    let id_name = get_field_name(id_field);
    let id_ty = &id_field.ty;

    // skip id field
    let update_fields = fields.iter().filter(|f| f!= &id_field );
    // a=?,b=?,c=? or a=$1,b=$2,c=$3
    let update_fields_str = update_fields.clone().enumerate()
        .map(|(i, f)| format!("{} = {}", get_field_name(f), db_placeholder(i+1)))
        .collect::<Vec<_>>()
        .join(",");
    // println!("update_fields_str: {}", update_fields_str);

    let update_fields = update_fields.flat_map(|f| &f.ident).collect::<Vec<_>>();
    let len = update_fields.len();
    let (pool, query_result) = db_pool_token();
    let placeholder = db_placeholder(1);
    let placeholder_u = db_placeholder(len+1);

    // by field
    let update_token = generate_update_field(&fields, &table_name, &id_column);
    let curd_by_field = generate_crud_by_field(&fields, &table_name,&update_fields_str,len);

    TokenStream::from(quote! {
        impl #struct_name {

            #curd_by_field
            #update_token

            pub async fn get(pool: &#pool, id: #id_ty) -> sqlx::Result<Self> {
                let sql = format!("SELECT * FROM {} WHERE {} = {}", #table_name, #id_name, #placeholder);
                sqlx::query_as::<_, Self>(&sql)
                .bind(id)
                .fetch_one(pool).await
            }

            pub async fn get_by(pool: &#pool, where_sql: &str) -> sqlx::Result<Self> {
                let mut sql = format!("SELECT * FROM {}", #table_name);
                if !where_sql.is_empty(){
                    sql = format!("{} WHERE {}", sql, where_sql);
                }
                sqlx::query_as::<_, Self>(&sql)
                .fetch_one(pool).await
            }

            pub async fn query_by(pool: &#pool, where_sql: &str) -> sqlx::Result<Vec<Self>> {
                let mut sql = format!("SELECT * FROM {}", #table_name);
                if !where_sql.is_empty(){
                    sql = format!("{} WHERE {}", sql, where_sql);
                }
                sqlx::query_as::<_, Self>(&sql)
                
                .fetch_all(pool).await
            }

            pub async fn insert(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let sql = format!("INSERT INTO {} ({}) values ({}) ", #table_name, #columns, #values);
                //RETURNING {}
                sqlx::query(&sql)
                #(
                    .bind(&self.#field_name_insert)
                )*
                .execute(pool).await
            }

            pub async fn upsert(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let sql = format!("REPLACE INTO {} ( {} ) values ({})", #table_name, #columns_all, #values_all);
                sqlx::query(&sql)
                #(
                    .bind(&self.#field_name_all)
                )*
                .execute(pool).await
            }

            pub async fn delete(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let mut sql = format!("DELETE FROM {} WHERE {}={}", #table_name,#id_name,#placeholder);
                
                sqlx::query(&sql)
                .bind(&self.#id_column)
                .execute(pool).await
            }

            pub async fn delete_by(pool: &#pool, where_sql: &str) -> sqlx::Result<#query_result> {
                let mut sql = format!("DELETE FROM {}", #table_name);
                if !where_sql.is_empty(){
                    sql = format!("{} WHERE {}", sql, where_sql);
                }
                sqlx::query(&sql)
                
                .execute(pool).await
            }

            pub async fn update(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let sql = format!("UPDATE {} SET {} WHERE {} = {}", #table_name, #update_fields_str, #id_name, #placeholder_u);
                sqlx::query(&sql)
                #(
                     .bind(&self.#update_fields) 
                )*
                .bind(&self.#id_column)
                .execute(pool).await
            }

            pub async fn update_by(&self, pool: &#pool, where_sql: &str) -> sqlx::Result<#query_result> {
                let mut sql = format!("UPDATE {} SET {} ", #table_name, #update_fields_str);
                if !where_sql.is_empty(){
                    sql = format!("{} WHERE {}", sql, where_sql);
                }
                sqlx::query(&sql)
                #(
                     .bind(&self.#update_fields) 
                )*
                .execute(pool).await
            }

        }
    })
}

// make string "?, ?, ?" or "$1, $2, $3"
fn question_marks(max: usize) -> String {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|i| db_placeholder(i))
        .collect::<Vec<String>>()
        .join(",")
}