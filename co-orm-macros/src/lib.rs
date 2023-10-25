/*
 * @Author: plucky
 * @Date: 2022-09-04 00:01:24
 * @LastEditTime: 2023-10-25 23:07:43
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



/// `#[derive(Crud)]`
/// generate get, get_by, query_by, update, delete, insert, insert_all
/// 
/// attributes:
/// 
/// `#[orm_pk]`
/// default first field is primary key or set.
/// 
/// `#[orm_seq]`
/// sequence field, auto increment. insert will ignore this field.
/// 
/// `#[orm_rename= "name"]`
/// rename table name or field name. 
/// default table name by struct name to_table_case: UserDetail => user_details. 
/// default field name by field name to_snake_case: UserDetail => user_detail. 
/// 
/// `#[orm_ignore]`
/// ignore field.
/// 
/// `#[orm_update]`
/// generate method update_xxx. 
/// 
/// `#[orm_by]`
/// generate query_by_field,update_by_field,delete_by_field.
/// 
#[proc_macro_derive(Crud, 
    attributes(
        orm_pk, // default first field is primary key or set
        orm_seq, // sequence field, auto increment. insert will ignore this field
        orm_update, // generate method update_xxx. 
        orm_rename, // rename table name or field name
        orm_ignore, // ignore field
        orm_by, // query_by_field,update_by_field,delete_by_field
    )
)]
pub fn sql_derive_crud(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    
    // if let Err(e) =  check_attributes_sql(&input.attrs){
    //     return e.to_compile_error().into();
    // }

    generate_crud(input)
    
}

/// impl sqlx::FromRow trait.
/// 
/// if use `#[derive(FromRow)]` macro, must use `#[derive(Crud)]` macro.
/// 
/// if you don't want to use `#[derive(co_orm::FromRow)]` macro, 
/// you can use `#[derive(sqlx::FromRow)]` macro or impl `sqlx::FromRow` trait.
/// 
/// if using sqlx::FromRow, `#[orm_ignore]` add `#[sql::defult]` .
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