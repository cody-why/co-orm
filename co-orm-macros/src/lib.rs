/*
 * @Author: plucky
 * @Date: 2022-09-04 00:01:24
 * @LastEditTime: 2024-08-03 12:47:01
 */

extern crate proc_macro;

use helper::check_attributes;
use impl_by_field::generate_impl_from_row;
use proc_macro::TokenStream;
use impl_crud::generate_crud;
use syn::{parse_macro_input, DeriveInput};


mod util;
mod impl_by_field;
mod db_type;
mod impl_crud;
mod helper;



/// `#[derive(Crud)]`
/// generate method: get, get_by, query, query_by, update, delete, insert, insert_all, query_page_by
/// 
/// attributes:
/// 
/// `#[co_orm(id)]`
/// default first field is primary key or set.
/// 
/// `#[co_orm(seq)]`
/// sequence field, auto increment. insert will skip this field.
/// 
/// `#[co_orm(rename="name")]`
/// rename table name or field name. 
/// default table name by struct name to_table_case: UserDetail => user_detail. 
/// default field name by field name to_snake_case: UserDetail => user_detail. 
/// 
/// `#[co_orm(skip)]`
/// ignore field. using sqlx::FromRow, skip need `#[co_orm(skip)]` and `#[sqlx(skip)]`
/// 
/// `#[co_orm(update)]`
/// generate method update_xxx. 
/// 
/// `#[co_orm(by)]`
/// generate qet_by_field, query_by_field, update_by_field, delete_by_field.
/// 
/// `#[co_orm(skip_insert)]`
/// insert will skip this field.
#[proc_macro_derive(Crud, 
    attributes(
        co_orm, // co_orm(id), co_orm(seq), co_orm(rename="name"), co_orm(skip), co_orm(update), co_orm(by),
        
    )
)]
pub fn sql_derive_crud(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    
    if let Err(e) =  check_attributes(&input.attrs){
       return e.to_compile_error().into();
    }

    generate_crud(input)
    
}

/// `#[derive(FromRow)]`
/// generate impl sqlx::FromRow for struct.
/// 
/// or use `#[derive(sqlx::FromRow)]`.
/// 
#[proc_macro_derive(FromRow)]
pub fn sql_derive_form_row(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);

    generate_impl_from_row(&input).into()

}

