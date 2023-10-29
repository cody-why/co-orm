/***
 * @Author: plucky
 * @Date: 2022-09-05 01:05:21
 * @LastEditTime: 2022-09-14 01:10:23
 * @Description: 
 */
#![allow(unused)]


use syn::{Field, DeriveInput, Type};
use inflector::Inflector;


/// skip field
pub(crate) fn is_skip(field: &Field) -> bool {
    // has_attribute_value(&field.attrs, "sql", "ignore")
    has_attribute(&field.attrs, "orm_ignore") |
    has_attribute_value(&field.attrs, "co_orm", "skip")
}

/// primary key
pub(crate) fn is_id(field: &Field) -> bool {
    has_attribute(&field.attrs, "orm_pk") |
    has_attribute_value(&field.attrs, "co_orm", "id")
}

pub(crate) fn is_seq(field: &Field) -> bool {
    has_attribute(&field.attrs, "orm_seq") |
    has_attribute_value(&field.attrs, "co_orm", "seq")
}

/// table_name
pub fn get_table_name(input: &DeriveInput) -> String {
    // to_table_case: UserDetail => user_details
    // to_snake_case: UserDetail => user_detail
   
    get_attribute_by_key(&input.attrs, "co_orm","rename").unwrap_or_else(|| {
        get_attribute_value(&input.attrs, "orm_rename").unwrap_or_else(|| {
            input.ident.to_string().to_snake_case()
        })
    })
}

/// field_name if rename
pub fn get_field_name(field: &Field) -> String {
    get_attribute_by_key(&field.attrs, "co_orm","rename").unwrap_or_else(|| {
        get_attribute_value(&field.attrs, "orm_rename").unwrap_or_else(|| {
            field.ident.as_ref().unwrap().to_string().to_snake_case()
        })
    })
    
}




/// `#[name(value)]` attribute value exist or not
pub(crate) fn has_attribute_value(attrs: &[syn::Attribute], name: &str, value: &str) -> bool {
    for attr in attrs.iter() {
        if !attr.path.is_ident(name){
            continue;
        }
        if let Ok(syn::Meta::List(list)) = attr.parse_meta(){
            for nested in list.nested.iter() {
                if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                    if path.is_ident(value) {
                        return true;
                    }
        
                }
            }

        }
          
    }
    false
}

/// `#[name]` attribute name exist or not
pub(crate) fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path.is_ident(name))
}

/// `#[name(key="val")]` Get the value of the name attribute by key
pub(crate) fn get_attribute_by_key(attrs: &[syn::Attribute], name: &str, key: &str) -> Option<String> {
    if let Some(Ok(syn::Meta::List(list))) = attrs.iter()
         .find(|a| a.path.is_ident(name))
         .map(|a| a.parse_meta()) {
         for nested in list.nested.iter() {
             if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                 if name_value.path.is_ident(key) {
                     if let syn::Lit::Str(lit_str) = &name_value.lit {
                         return Some(lit_str.value());
                     }
                 }
             }         
        } 
     };
    None
}


/// `#[name = "0b{:08b}"]` Get the value of the name attribute
pub(crate) fn get_attribute_value(attrs: &[syn::Attribute], key: &str)-> Option<String>{
    for attr in attrs {
        // Meta是NameValue(MetaNameValue)的属性
        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        }))=attr.parse_meta() {
            if path.is_ident(key) {
                if let syn::Lit::Str(ref s) = lit {
                    return Some(s.value());
                }
            }
        }

    }
    None
}

/// whether `Option<inner_type>` returns (whether Option, inner_type).
pub(crate) fn get_option_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Option")
}

#[allow(dead_code)]
/// whether `Vec<inner_type>` returns (whether Vec, inner_type).
pub(crate) fn get_vec_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Vec")
}


/// whether inner_type,such as: Option<String>,Vec<String>
/// returns (whether, inner_type).
pub(crate) fn get_inner_type<'a>(ty: &'a Type, name:&str) -> (bool, &'a Type) {
    // syn::Type::Path(ref path) {segments[0].ident="Option"}
    if let syn::Type::Path(ref path) = ty{
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(ty)) = args.first() {
                        return (true, ty);
                        
                    }
                }

            }
        }
    }
    (false, ty)
}


#[test]
fn test_table_name(){
    let name = "permission";
    let table_name = name.to_snake_case();
    println!("table_name: {}", table_name);
}