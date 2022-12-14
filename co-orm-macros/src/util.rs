/***
 * @Author: plucky
 * @Date: 2022-09-05 01:05:21
 * @LastEditTime: 2022-09-14 01:10:23
 * @Description: 
 */
#![allow(unused)]


use syn::{Field, DeriveInput, Type};
use inflector::Inflector;


/// ignore field
pub(crate) fn is_ignore(field: &Field) -> bool {
    // has_attribute_value(&field.attrs, "sql", "ignore")
    has_attribute(&field.attrs, "orm_ignore")
}

/// primary key
pub(crate) fn is_id(field: &Field) -> bool {
    has_attribute(&field.attrs, "orm_pk")
}

pub(crate) fn is_seq(field: &Field) -> bool {
    has_attribute(&field.attrs, "orm_seq")
}

/// table_name
pub fn get_table_name(input: &DeriveInput) -> String {
    // to_table_case: UserDetail => user_details
    // to_snake_case: UserDetail => user_detail
    let name = get_attribute_value(&input.attrs, "orm_rename").unwrap_or_else(|| {
        input.ident.to_string().to_snake_case()
    });
    
    name
}

/// field_name if rename
pub fn get_field_name(field: &Field) -> String {
    let name = get_attribute_value(&field.attrs, "orm_rename").unwrap_or_else(|| {
        field.ident.as_ref().unwrap().to_string().to_snake_case()
    });
    
    name
}


/// `#[name(value)]` attribute value exist or not
pub fn has_attribute_value(attrs: &Vec<syn::Attribute>, name: &str, value: &str) -> bool {
    for attr in attrs.iter() {
        if !attr.path.is_ident(name){
            continue;
        }
        if let Ok(list) = attr.parse_meta(){
            if let syn::Meta::List(list) = list {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                        if path.is_ident(value) {
                            return true;
                        }
                        
                    }
                }
            }

        }
          
    }
    false
}

/// `#[name]` attribute name exist or not
pub fn has_attribute(attrs: &Vec<syn::Attribute>, name: &str) -> bool {
    // for attr in attrs.iter() {
    //     if attr.path.is_ident(name){
    //         return  true;
    //     }
    // }
    // false

    attrs.iter().any(|attr| attr.path.is_ident(name))
}

/// `#[name(key="val")]` Get the value of the name attribute by key
pub fn get_attribute_by_key(attrs: &Vec<syn::Attribute>, name: &str, key: &str) -> Option<String> {
    match attrs.iter()
        .find(|a| a.path.is_ident(name))
        .map(|a| a.parse_meta())
        {
            Some(Ok(syn::Meta::List(list))) => {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident(key) {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }

            }
            _ => {}
        };
    None
}


/// `#[name = "0b{:08b}"]` Get the value of the name attribute
pub fn get_attribute_value(attrs: &Vec<syn::Attribute>, key: &str)-> Option<String>{
    for attr in attrs {
        // Meta???NameValue(MetaNameValue)?????????
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
pub fn get_option_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Option")
}

#[allow(dead_code)]
/// whether `Vec<inner_type>` returns (whether Vec, inner_type).
pub fn get_vec_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Vec")
}


/// whether inner_type,such as: Option<String>,Vec<String>
/// returns (whether, inner_type).
fn get_inner_type<'a>(ty: &'a Type, name:&str) -> (bool, &'a Type) {
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

// Check that the attributes are correct
// pub fn check_attributes_sql(attrs: &Vec<syn::Attribute>) -> syn::Result<()> {
//     for attr in attrs.iter() {
//         if !attr.path.is_ident("sql"){
//             continue;
//         }
//         if let Ok(list) = attr.parse_meta(){
//             if let syn::Meta::List(list) = list {
//                 for nested in list.nested.iter() {
//                     // #[sql(id)]
//                     if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
//                         match path.get_ident() {
//                             Some(ident) => {
//                                 let ident = ident.to_string();
//                                 // println!("ident: {}", ident);
//                                 if ident != "ignore" && ident != "id" && ident != "table_name" {
//                                     return Err(syn::Error::new_spanned(
//                                         attr,
//                                         "expected #[sql(id)] or #[sql(ignore)]",
//                                     ));
//                                 }
//                             }
//                             None => {
//                             }
//                         }
                            
                        
                        
//                     }

//                     // #[sql(table_name='')]
//                     if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
//                         if !name_value.path.is_ident("table_name") {
//                             return Err(syn::Error::new_spanned(
//                                 attr,
//                                 "expected #[sql(table_name='?')]",
//                             ));
//                         }
//                     }
//                 }
//             }

//         }
            
//     }
   
   
//     Ok(())
// }


