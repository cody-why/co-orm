/*
 * @Author: plucky
 * @Date: 2023-10-29 15:56:49
 * @LastEditTime: 2024-08-03 12:43:53
 */

use std::collections::HashSet;

use inflector::Inflector;
use syn::{DeriveInput, Field};

use crate::{db_type::db::*, util::*};

/// skip field
/// `#[co_orm(skip)]`
pub(crate) fn is_skip(field: &Field) -> bool {
    // has_attribute(&field.attrs, "orm_ignore") |
    has_attribute_value(&field.attrs, "co_orm", "skip") | has_attribute_value(&field.attrs, "sqlx", "skip")
}

/// primary key
/// `#[co_orm(id)]`
pub(crate) fn is_id(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "id")
}

pub(crate) fn is_seq(field: &Field) -> bool {
    has_attribute_value(&field.attrs, "co_orm", "seq") | has_attribute_value(&field.attrs, "co_orm", "skip_insert")
}

/// table_name
/// `#[co_orm(rename = "users")]`
pub(crate) fn get_table_name(input: &DeriveInput) -> String {
    // to_table_case: UserDetail => user_details
    // to_snake_case: UserDetail => user_detail

    get_attribute_by_key(&input.attrs, "co_orm", "rename").unwrap_or_else(|| input.ident.to_string().to_snake_case())
}

/// field_name if rename
/// `#[co_orm(rename = "name")]`
pub(crate) fn get_field_name(field: &Field) -> String {
    get_attribute_by_key(&field.attrs, "co_orm", "rename")
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string().to_snake_case())
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
    itr.into_iter().map(db_placeholder).collect::<Vec<String>>().join(",")
}

#[allow(unused)]
pub(crate) fn check_attributes(attrs: &[syn::Attribute]) -> Result<(), syn::Error> {
    // 检查属性是否 co_orm(id), co_orm(seq), co_orm(rename="name"), co_orm(skip), co_orm(update), co_orm(by),
    let valid_attrs: HashSet<_> = ["rename", "id", "by", "seq", "skip", "update"].iter().cloned().collect();

    for attr in attrs {
        if attr.path().is_ident("co_orm") {
            let pass = attr.parse_nested_meta(|meta| {
                // println!("path: {:?}", meta.path);
                let ident = meta.path.get_ident().unwrap().to_string();
                if valid_attrs.contains(ident.as_str()) {
                    return Ok(());
                }
                Err(meta.error("unrecognized repr"))
            });

            if let Err(e) = pass {
                if e.to_string().contains("unrecognized repr") {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_table_name() {
    let name = "permission";
    let table_name = name.to_snake_case();
    println!("table_name: {}", table_name);
}
