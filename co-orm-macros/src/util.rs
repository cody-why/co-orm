/***
 * @Author: plucky
 * @Date: 2022-09-05 01:05:21
 * @LastEditTime: 2022-09-14 01:10:23
 * @Description: 
 */
#![allow(unused)]


use syn::{Field, DeriveInput, Type, LitStr};
use inflector::Inflector;



/// `#[name(value)]` attribute value exist or not
pub(crate) fn has_attribute_value(attrs: &[syn::Attribute], name: &str, value: &str) -> bool {
    // println!("name: {}, value: {}", name, value);
    for attr in attrs.iter() {
        if !attr.path().is_ident(name){
            continue;
        }

        let f = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(value) {
                return Ok(());
            }
            Err(meta.error("attribute value not found"))
        });
        if f.is_ok(){
            return true;
        }
        
    }
    false
}

/// `#[name]` attribute name exist or not
pub(crate) fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}

/// `#[name(key="val")]` Get the value of the name attribute by key
pub(crate) fn get_attribute_by_key(attrs: &[syn::Attribute], name: &str, key: &str) -> Option<String> {
    let mut v: Option<String> = None;
    for attr in attrs.iter() {
        if !attr.path().is_ident(name){
            continue;
        }
      
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(key) {
                let value = meta.value()?;   // this parses the `=`
                let s: LitStr = value.parse()?;  // this parses `"val"`
                v = Some(s.value());
                return  Ok(());
            }
            Err(meta.error("attribute value not found"))
        }).ok();
    }
    v

}


/// `#[name = "0b{:08b}"]` Get the value of the name attribute
pub(crate) fn get_attribute_value(attrs: &[syn::Attribute], key: &str)-> Option<String>{
    for attr in attrs {
        if attr.path().is_ident(key) {
            let value = attr.parse_args::<syn::LitStr>().unwrap();
            return Some(value.value());
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

