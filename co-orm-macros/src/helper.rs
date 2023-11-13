/*
 * @Author: plucky
 * @Date: 2023-10-29 15:56:49
 * @LastEditTime: 2023-11-13 08:11:30
 */

use inflector::Inflector;
use syn::{Field, DeriveInput};

use crate::{util::*, db_type::db_placeholder};


/// skip field
/// `#[co_orm(skip)]`
pub(crate) fn is_skip(field: &Field) -> bool {
    // has_attribute(&field.attrs, "orm_ignore") |
    has_attribute_value(&field.attrs, "co_orm", "skip") |
    has_attribute_value(&field.attrs, "sqlx", "skip")
}

/// primary key
/// `#[co_orm(id)]`
pub(crate) fn is_id(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "id")
}

pub(crate) fn is_seq(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "seq")
}

/// table_name
/// `#[co_orm(rename = "users")]`
pub(crate) fn get_table_name(input: &DeriveInput) -> String {
    // to_table_case: UserDetail => user_details
    // to_snake_case: UserDetail => user_detail
   
    get_attribute_by_key(&input.attrs, "co_orm","rename").unwrap_or_else(|| {
        input.ident.to_string().to_snake_case()
    })
}

/// field_name if rename
/// `#[co_orm(rename = "name")]`
pub(crate) fn get_field_name(field: &Field) -> String {
    get_attribute_by_key(&field.attrs, "co_orm","rename").unwrap_or_else(|| {
        field.ident.as_ref().unwrap().to_string().to_snake_case()
    })
    
}

/// `#[co_orm(update)]`
pub(crate) fn has_attribute_update(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "update")
}

/// `#[co_orm(by)]`
pub(crate) fn has_attribute_by(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "by")
}

// make string "?, ?, ?" or "$1, $2, $3"
pub(crate) fn question_marks(max: usize) -> String {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(db_placeholder)
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn test_table_name(){
    let name = "permission";
    let table_name = name.to_snake_case();
    println!("table_name: {}", table_name);
}