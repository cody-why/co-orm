/*
 * @Author: plucky
 * @Date: 2022-10-19 17:45:59
 * @LastEditTime: 2023-10-29 11:58:09
 * @Description: 
 */

use quote::{format_ident, quote, __private::TokenStream};
use syn::Field;
use crate::{util::*, db_type::*};

fn has_attribute_update(field: &Field) -> bool {
    // has_attribute_value(&field.attrs, "sql", "ignore")
    has_attribute(&field.attrs, "orm_update") |
    has_attribute_value(&field.attrs, "co_orm", "update")
}

fn has_attribute_by(field: &Field) -> bool {
    // has_attribute_value(&field.attrs, "sql", "ignore")
    has_attribute(&field.attrs, "orm_by") |
    has_attribute_value(&field.attrs, "co_orm", "by")
}

/// only update one field
pub fn generate_update_field(fields: &Vec<&Field>, table_name:&str, id_column: &syn::Ident) -> TokenStream {
    let update_tokens = fields.iter()
        .filter_map(|field| {
            if has_attribute_update(field) {

                let field_ident = field.ident.as_ref().unwrap();
                
                let fn_name = format_ident!("update_{}",field_ident);
                let value_token = quote! { &self.#field_ident };
  
                let id_name = id_column.to_string();
                
                let (pool, query_result) = db_pool_token();
                let placeholder = db_placeholder(1);
                
                let field_name = get_attribute_value(&field.attrs, "orm_rename").unwrap_or(field_ident.to_string());
                let set_sql = format!("{} = {}", field_name, placeholder);
                let doc = format!("only update one field {}", field_ident);
                let placeholder = db_placeholder(2);

                let code = quote!{
                    #[doc = #doc]
                    pub async fn #fn_name(&self, pool: &#pool) ->sqlx::Result<#query_result> {
                        let sql = format!("UPDATE {} SET {} WHERE {} = {}", #table_name, #set_sql, #id_name, #placeholder);
                        sqlx::query(&sql)
                        .bind(#value_token)
                        .bind(&self.#id_column)
                        .execute(pool).await
                        
                    }
                };
                Some(code)
            } else {
                None
            }
        }).collect::<Vec<_>>();

    quote!{
        #(#update_tokens)*
    }
}



/// query_by_field,update_by_field,delete_by_field
pub fn generate_crud_by_field(fields: &Vec<&Field>, table_name:&str, update_fields_str:&str,len:usize) -> TokenStream {
    let generate_tokens = fields.iter()
        .filter_map(|field| {
            if has_attribute_by(field) {

                let field_ident = field.ident.as_ref().unwrap();
                let (_, field_type) = get_option_type(&field.ty);
                let field_name = get_attribute_value(&field.attrs, "orm_rename").unwrap_or(field_ident.to_string());
                
                let fn_query = format_ident!("query_by_{}",field_ident);
                let fn_delete = format_ident!("delete_by_{}",field_ident);
                let fn_update = format_ident!("update_by_{}",field_ident);

                let (pool, query_result) = db_pool_token();
                let placeholder = db_placeholder(1);
                let placeholder_u = db_placeholder(len+1);
                
                let code = quote!{
                    // #[doc = #doc]
                    pub async fn #fn_query(pool: &#pool, value:#field_type) ->sqlx::Result<Vec<Self>>{
                        let sql = format!("SELECT * FROM {} WHERE {} = {}", #table_name, #field_name, #placeholder);
                        sqlx::query_as::<_, Self>(&sql)
                        .bind(value)
                        .fetch_all(pool).await
                        
                    }

                    
                    pub async fn #fn_delete(pool: &#pool, value:#field_type) ->sqlx::Result<#query_result> {
                        let sql = format!("DELETE FROM {} WHERE {} = {}", #table_name, #field_name, #placeholder);
                        sqlx::query(&sql)
                        .bind(value)
                        .execute(pool).await
                        
                    }

                    pub async fn #fn_update(pool: &#pool, value:#field_type) ->sqlx::Result<#query_result> {
                        let sql = format!("UPDATE {} SET {} WHERE {} = {}", #table_name, #update_fields_str, #field_name, #placeholder_u);
                        sqlx::query(&sql)
                        .bind(value)
                        .execute(pool).await
                        
                    }


                };
                Some(code)
            } else {
                None
            }
        }).collect::<Vec<_>>();

    quote!{
        #(#generate_tokens)*
    }
}

/// impl sqlx::FromRow
pub fn generate_impl_from_row(fields: &Vec<&Field>, struct_name:&syn::Ident) -> TokenStream {
    let fields = fields.iter().map(|field| -> TokenStream {
        let field_ident = field.ident.as_ref().unwrap();
        // ignore or rename or option
        if is_skip(field) {
            quote!{#field_ident: Default::default(),}
        } else {
            let field_name = get_field_name(field);
            let (is_option, _field_type) = get_option_type(&field.ty);
            if is_option {
                return quote!{#field_ident: sqlx::Row::try_get(row,#field_name).ok(),}
            } 
            quote!{#field_ident: sqlx::Row::try_get(row,#field_name)?,}

        }
       
    });

    let row = db_row_token();
    quote!{
        // use sqlx::Row;
        impl sqlx::FromRow<'_, #row> for #struct_name {
            fn from_row(row: &#row) -> sqlx::Result<Self> {
                Ok(Self {
                    #(#fields)*
                   
                })
            }
        }
    }
}