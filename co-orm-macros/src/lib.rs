/*
 * @Author: plucky
 * @Date: 2022-09-04 00:01:24
 * @LastEditTime: 2023-10-29 16:08:07
 * @Description: 
 */


extern crate proc_macro;

use impl_by_field::*;
use proc_macro::TokenStream;
use impl_crud::generate_crud;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};


mod util;
mod impl_by_field;
mod db_type;
mod impl_crud;
mod helper;



/// `#[derive(Crud)]`
/// generate method: get, get_by, query, query_by, update, delete, insert, insert_all.
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
/// ignore field.
/// 
/// `#[co_orm(update)]`
/// generate method update_xxx. 
/// 
/// `#[co_orm(by)]`
/// generate query_by_field,update_by_field,delete_by_field.
/// 
#[proc_macro_derive(Crud, 
    attributes(
        co_orm, // co_orm(id), co_orm(seq), co_orm(rename="name"), co_orm(skip), co_orm(update), co_orm(by),
        // orm_pk, // default first field is primary key or set
        // orm_seq, // sequence field, auto increment. insert will skip this field
        // orm_rename, // rename table name or field name
        // orm_ignore, // ignore field
        // orm_update, // generate method update_xxx. 
        // orm_by, // query_by_field,update_by_field,delete_by_field
    )
)]
pub fn sql_derive_crud(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    
    // if let Err(e) =  check_attributes_sql(&input.attrs){
    //     return e.to_compile_error().into();
    // }

    generate_crud(input)
    
}
/// `#[derive(FromRow)]`
/// impl sqlx::FromRow trait.
/// 
/// or use `#[derive(sqlx::FromRow)]` macro or impl `sqlx::FromRow` trait.
/// 
/// if using sqlx::FromRow, change `#[co_orm(skip)]` to `#[sqlx(skip)]` .
/// 
#[proc_macro_derive(FromRow)]
pub fn sql_derive_form_row(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {fields: Fields::Named(fields),..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let fields_all = fields.iter().collect::<Vec<_>>();
    let generate_from_row = generate_impl_from_row(&fields_all, struct_name);
    TokenStream::from(quote! {
        #generate_from_row
    })

}