/*
 * @Author: plucky
 * @Date: 2022-10-22 18:08:45
 * @LastEditTime: 2023-12-04 16:08:28
 * @Description:
 */

use crate::db_type::*;
use crate::helper::*;
use crate::impl_by_field::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

///  generate_crud
pub fn generate_crud(input: DeriveInput) -> TokenStream {
    let table_name = get_table_name(&input);
    // println!("table_name: {}", table_name);

    let struct_name = &input.ident;

    let fields1 = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    //  for f in fields1 {
    //     if let Err(e) = check_attributes(&f.attrs) {
    //         return e.to_compile_error().into();
    //     }
    //  }

    // filter skip ignore fields
    let fields = fields1.iter().filter(|f| !is_skip(f)).collect::<Vec<_>>();

    // insert skip seq field
    let fields_insert = fields.iter().filter(|f| !is_seq(f)).collect::<Vec<_>>();

    let field_name_insert = fields_insert.iter().map(|field| &field.ident).collect::<Vec<_>>();
    
    // insert (a,b,c) values (?,?,?)
    let insert_columns = fields_insert
        .iter()
        .map(|field| format!("`{}`",get_field_name(field)))
        .collect::<Vec<_>>()
        .join(",");

    let values = question_marks(fields_insert.len());

    // let fields_list = quote! {
    //     #(#column_name),*
    // };

    let select_columns = fields
        .iter()
        .map(|field| format!("`{}`",get_field_name(field)))
        .collect::<Vec<_>>()
        .join(",");

    // upsert use all fields except ignore
    // let field_name_all = fields.iter().map(|field| {
    //     &field.ident
    // }).collect::<Vec<_>>();
    // with id field

    // let values_all = question_marks(fields.len());

    // find `orm_pk` or default the first field as the "id" column
    let id_field = fields
        .iter()
        .find(|f| is_id(f))
        .unwrap_or_else(|| fields.first().unwrap());
    let id_column = id_field.ident.as_ref().unwrap();
    let id_name = get_field_name(id_field);
    let id_ty = &id_field.ty;

    // skip id field
    let update_fields = fields.iter().filter(|f| f != &id_field);
    // a=?,b=?,c=? or a=$1,b=$2,c=$3
    let update_fields_str = update_fields
        .clone()
        .enumerate()
        .map(|(i, f)| format!("`{}` = {}", get_field_name(f), db_placeholder(i + 1)))
        .collect::<Vec<_>>()
        .join(",");
    // println!("update_fields_str: {}", update_fields_str);

    let update_fields = update_fields.flat_map(|f| &f.ident).collect::<Vec<_>>();
    let len = update_fields.len();
    let (pool, query_result) = db_pool_token();
    let placeholder = db_placeholder(1);
    let placeholder_u = db_placeholder(len + 1);
    let db_arguments = db_arguments_token();

    // by field
    let update_token = generate_update_field(&fields, &table_name, id_column);
    let curd_by_field = generate_crud_by_field(&fields, &table_name, &update_fields_str, &select_columns, len);

    TokenStream::from(quote! {
        impl #struct_name {
            #curd_by_field
            #update_token

            /// get by id
            ///
            /// Example:
            /// ```` no_run
            /// User::get(&pool, 1).await
            /// ````
            pub async fn get(pool: &#pool, id: #id_ty) -> sqlx::Result<Self> {
                let sql = format!("SELECT * FROM {} WHERE {} = {}", #table_name, #id_name, #placeholder);
                sqlx::query_as::<_, Self>(&sql)
                   .bind(id)
                   .fetch_one(pool).await
            }

            /// get by where sql
            /// Example:
            /// ```` no_run
            /// User::get_by(&pool, "where id=?", sql_args!(1)).await
            /// ````
            pub async fn get_by(pool: &#pool, where_sql: impl AsRef<str>, args: #db_arguments) -> sqlx::Result<Self> {
                let sql = format!("SELECT * FROM {} {}", #table_name, where_sql.as_ref());
                // sqlx::query_as::<_, Self>(&sql)
                sqlx::query_as_with::<_,Self,_>(&sql, args)
                   .fetch_one(pool).await
            }

            /// query
            pub async fn query(pool: &#pool) -> sqlx::Result<Vec<Self>> {
                let sql = format!("SELECT * FROM {}", #table_name);

                sqlx::query_as::<_, Self>(&sql)
                   .fetch_all(pool).await
            }

            /// query by where sql
            ///
            /// Example:
            /// ```` no_run
            /// User::query_by(&pool, "where id=?", sql_args!(1)).await
            /// ````
            pub async fn query_by(pool: &#pool, where_sql: impl AsRef<str>, args: #db_arguments) -> sqlx::Result<Vec<Self>> {
                let sql = format!("SELECT * FROM {} {}", #table_name, where_sql.as_ref());

                // sqlx::query_as::<_, Self>(&sql)
                sqlx::query_as_with::<_,Self,_>(&sql, args)
                   .fetch_all(pool).await
            }

            /// insert
            pub async fn insert(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let sql = format!("INSERT INTO {} ({}) values ({}) ", #table_name, #insert_columns, #values);
                // RETURNING {}
                sqlx::query(&sql)
                #(
                    .bind(&self.#field_name_insert)
                )*
                .execute(pool).await
            }

            // pub async fn upsert(&self, pool: &#pool) -> sqlx::Result<#query_result> {
            //     let sql = format!("REPLACE INTO {} ({}) values ({})", #table_name, #columns_all, #values_all);
            //     sqlx::query(&sql)
            //     #(
            //         .bind(&self.#field_name_all)
            //     )*
            //     .execute(pool).await
            // }

            /// delete by id
            pub async fn delete(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let mut sql = format!("DELETE FROM {} WHERE {}={}", #table_name,#id_name,#placeholder);

                sqlx::query(&sql)
                   .bind(&self.#id_column)
                   .execute(pool).await
            }

            /// delete by where sql
            ///
            /// Example:
            /// ```` no_run
            /// User::delete_by(&pool, "where id=?", sql_args!(1)).await
            /// ````
            pub async fn delete_by(pool: &#pool, where_sql: impl AsRef<str>, args: #db_arguments) -> sqlx::Result<#query_result> {
                let sql = format!("DELETE FROM {} {}", #table_name, where_sql.as_ref());

                sqlx::query_with(&sql,args)
                   .execute(pool).await
            }

            /// update by id
            pub async fn update(&self, pool: &#pool) -> sqlx::Result<#query_result> {
                let sql = format!("UPDATE {} SET {} WHERE {} = {}", #table_name, #update_fields_str, #id_name, #placeholder_u);
                sqlx::query(&sql)
                #(
                     .bind(&self.#update_fields)
                )*
                .bind(&self.#id_column)
                .execute(pool).await
            }

            /// update by where sql
            ///
            /// Example:
            /// ```` no_run
            /// User::update_by(&pool, "where id=1").await
            /// ````
            pub async fn update_by(&self, pool: &#pool, where_sql: impl AsRef<str>) -> sqlx::Result<#query_result> {
                let sql = format!("UPDATE {} SET {} {}", #table_name, #update_fields_str, where_sql.as_ref());
                // todo!
                sqlx::query(&sql)
                #(
                     .bind(&self.#update_fields)
                )*
                .execute(pool).await
            }

            /// insert all list
            pub async fn insert_all(pool: &#pool, list: Vec<Self>) -> sqlx::Result<u64> {
                let sql = format!("INSERT INTO {} ({}) ", #table_name, #insert_columns);
                let mut qb = sqlx::QueryBuilder::new(sql);

                qb.push_values(list, |mut q, one| {
                    // q.push_bind(one.name).push_bind(one.password);
                    q
                    #(
                        .push_bind(one.#field_name_insert)
                    )*
                    ;
                });
                let id = qb.build().execute(pool).await?;

                Ok(id.rows_affected())
            }

           /// guery page by where sql
           ///
           /// Example:
           /// ```` no_run
           /// let r = User::query_page_by(&pool, "where id>?", sql_args!(1), 1, 10).await
           /// if let Ok((count, list)) = r {
           ///     println!("count: {}, list: {:?}", count, list);
           /// }
           /// ````
           pub async fn query_page_by(pool: &#pool, where_sql: impl AsRef<str>, args: #db_arguments, page: i32, page_size: i32) -> sqlx::Result<(i64, Vec<Self>)> {
                let sql = format!("SELECT * FROM {} {}", #table_name, where_sql.as_ref());

                let total = sqlx::query_scalar_with::<_,i64,_>(&format!("select count(*) from ({}) as c", sql),args.clone())
                   .fetch_one(pool).await?;
                let sql = format!("{} LIMIT {} OFFSET {}", sql, page_size, page_size * (page - 1));
                sqlx::query_as_with::<_, Self,_>(&sql,args).fetch_all(pool).await.map(|list| (total, list))
           }
       }
    })
}
