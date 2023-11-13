
#![allow(unused)]

use syn::{Type, LitStr};


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
    let mut val: Option<String> = None;
    for attr in attrs.iter() {
        if !attr.path().is_ident(name){
            continue;
        }
      
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(key) {
                let value = meta.value()?;   // this parses the `=`
                let v: LitStr = value.parse()?;  // this parses `"val"`
                val = Some(v.value());
                return  Ok(());
            }
            Err(meta.error("attribute value not found"))
        }).ok();
    }
    val

}


/// `#[name = "value"]` Get the value of the name attribute
pub(crate) fn get_attribute_value(attrs: &[syn::Attribute], key: &str)-> Option<String>{
    for attr in attrs {
        if attr.path().is_ident(key) {
            // #[name = "value"]
           let r = attr.meta.require_name_value();
           if let Ok(v) = r {
                if let  syn::Expr::Lit(v) = &v.value {
                    match &v.lit {
                        syn::Lit::Str(s) => {
                            return Some(s.value());
                        }
                        syn::Lit::Int(i) => {
                            return Some(i.to_string());
                        }
                        syn::Lit::Float(f) => {
                            return Some(f.to_string());
                        }
                        syn::Lit::Bool(b) => {
                            return Some(b.value().to_string());
                        }
                       
                        _ => {}
                        
                    }
                }
            
           }
          
        }
    }
    None
}

/// `#[name(arg)]` Get the arg of the name attribute
pub(crate) fn get_attribute_arg(attrs: &[syn::Attribute], key: &str)-> Option<String>{
    for attr in attrs {
        if attr.path().is_ident(key) {
           let r = attr.parse_args::<syn::Lit>();
           if let Ok(v) = r {
                println!("v: {:?}", v);
                match &v {
                    syn::Lit::Str(s) => {
                        return Some(s.value());
                    }
                    syn::Lit::Int(i) => {
                        return Some(i.to_string());
                    }
                    syn::Lit::Float(f) => {
                        return Some(f.to_string());
                    }
                    syn::Lit::Bool(b) => {
                        return Some(b.value().to_string());
                    }
                    _ => {}
                    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let attr: syn::Attribute = syn::parse_quote!(#[name = "0b{:08b}"]);
        // println!("attr: {:?}", attr);
        let v = get_attribute_value(&[attr], "name");
        println!("v: {:?}", v);
        // assert_eq!(v, Some("0b{:08b}".to_string()));

        let attr: syn::Attribute = syn::parse_quote!(#[name("yes")]);
        let v = get_attribute_arg(&[attr], "name");
        println!("v: {:?}", v);
    }
}