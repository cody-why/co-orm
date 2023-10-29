/*
 * @Author: plucky
 * @Date: 2023-10-29 15:56:49
 * @LastEditTime: 2023-10-29 16:11:41
 */

use inflector::Inflector;
use syn::{Field, DeriveInput};

use crate::util::*;


/// skip field
pub(crate) fn is_skip(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "skip")
}

/// primary key
pub(crate) fn is_id(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "id")
}

pub(crate) fn is_seq(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "seq")
}

/// table_name
pub(crate) fn get_table_name(input: &DeriveInput) -> String {
    // to_table_case: UserDetail => user_details
    // to_snake_case: UserDetail => user_detail
   
    get_attribute_by_key(&input.attrs, "co_orm","rename").unwrap_or_else(|| {
        input.ident.to_string().to_snake_case()
    })
}

/// field_name if rename
pub(crate) fn get_field_name(field: &Field) -> String {
    get_attribute_by_key(&field.attrs, "co_orm","rename").unwrap_or_else(|| {
        field.ident.as_ref().unwrap().to_string().to_snake_case()
    })
    
}

pub(crate) fn has_attribute_update(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "update")
}

pub(crate) fn has_attribute_by(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "by")
}


#[test]
fn test_table_name(){
    let name = "permission";
    let table_name = name.to_snake_case();
    println!("table_name: {}", table_name);
}