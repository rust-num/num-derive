// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "proc-macro"]
#![doc(html_root_url = "https://docs.rs/num-derive/0.2")]
#![recursion_limit="256"]

//! Procedural macros to derive numeric traits in Rust.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! num-traits = "0.2"
//! num-derive = "0.2"
//! ```
//!
//! Then you can derive traits on your own types:
//!
//! ```rust
//! #[macro_use]
//! extern crate num_derive;
//!
//! #[derive(FromPrimitive, ToPrimitive)]
//! enum Color {
//!     Red,
//!     Blue,
//!     Green,
//! }
//! # fn main() {}
//! ```

extern crate proc_macro;

extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{Data, Fields, Ident};

// Within `exp`, you can bring things into scope with `extern crate`.
//
// We don't want to assume that `num_traits::` is in scope - the user may have imported it under a
// different name, or may have imported it in a non-toplevel module (common when putting impls
// behind a feature gate).
//
// Solution: let's just generate `extern crate num_traits as _num_traits` and then refer to
// `_num_traits` in the derived code.  However, macros are not allowed to produce `extern crate`
// statements at the toplevel.
//
// Solution: let's generate `mod _impl_foo` and import num_traits within that.  However, now we
// lose access to private members of the surrounding module.  This is a problem if, for example,
// we're deriving for a newtype, where the inner type is defined in the same module, but not
// exported.
//
// Solution: use the dummy const trick.  For some reason, `extern crate` statements are allowed
// here, but everything from the surrounding module is in scope.  This trick is taken from serde.
fn dummy_const_trick<T: quote::ToTokens>(
    trait_: &str,
    name: &proc_macro2::Ident,
    exp: T,
) -> proc_macro2::TokenStream {
    let dummy_const = Ident::new(
        &format!(
            "_IMPL_NUM_{}_FOR_{}",
            trait_.to_uppercase(),
            format!("{}", name).to_uppercase()
        ),
        Span::call_site(),
    );
    quote! {
        const #dummy_const: () = {
            #exp
        };
    }
}

// If `data` is a newtype, return the type it's wrapping.
fn newtype_inner(data: &syn::Data) -> Option<syn::Type> {
    match data {
        &Data::Struct(ref s) => match s.fields {
            Fields::Unnamed(ref fs) => {
                if fs.unnamed.len() == 1 {
                    Some(fs.unnamed[0].ty.clone())
                } else {
                    None
                }
            }
            Fields::Named(ref fs) => {
                if fs.named.len() == 1 {
                    panic!("num-derive doesn't know how to handle newtypes with named fields yet. \
                           Please use a tuple-style newtype, or submit a PR!");
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

/// Derives [`num_traits::FromPrimitive`][from] for simple enums and newtypes.
///
/// [from]: https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html
///
/// # Examples
///
/// Simple enums can be derived:
///
/// ```rust
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(FromPrimitive)]
/// enum Color {
///     Red,
///     Blue,
///     Green = 42,
/// }
/// # fn main() {}
/// ```
///
/// Enums that contain data are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(FromPrimitive)]
/// enum Color {
///     Rgb(u8, u8, u8),
///     Hsv(u8, u8, u8),
/// }
/// # fn main() {}
/// ```
///
/// Structs are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
/// #[derive(FromPrimitive)]
/// struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
/// # fn main() {}
/// ```
#[proc_macro_derive(FromPrimitive)]
pub fn from_primitive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let impl_ = if let Some(inner_ty) = newtype_inner(&ast.data) {
        let i128_fns = if cfg!(has_i128) {
            quote! {
                fn from_i128(n: i128) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_i128(n).map(#name)
                }
                fn from_u128(n: u128) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_u128(n).map(#name)
                }
            }
        } else {
            quote! {}
        };

        quote! {
            extern crate num_traits as _num_traits;
            impl _num_traits::FromPrimitive for #name {
                fn from_i64(n: i64) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_i64(n).map(#name)
                }
                fn from_u64(n: u64) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_u64(n).map(#name)
                }
                fn from_isize(n: isize) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_isize(n).map(#name)
                }
                fn from_i8(n: i8) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_i8(n).map(#name)
                }
                fn from_i16(n: i16) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_i16(n).map(#name)
                }
                fn from_i32(n: i32) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_i32(n).map(#name)
                }
                fn from_usize(n: usize) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_usize(n).map(#name)
                }
                fn from_u8(n: u8) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_u8(n).map(#name)
                }
                fn from_u16(n: u16) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_u16(n).map(#name)
                }
                fn from_u32(n: u32) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_u32(n).map(#name)
                }
                fn from_f32(n: f32) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_f32(n).map(#name)
                }
                fn from_f64(n: f64) -> Option<Self> {
                    <#inner_ty as _num_traits::FromPrimitive>::from_f64(n).map(#name)
                }
                #i128_fns
            }
        }
    } else {
        let variants = match ast.data {
            Data::Enum(ref data_enum) => &data_enum.variants,
            _ => panic!(
                "`FromPrimitive` can be applied only to enums and newtypes, {} is neither",
                name
            ),
        };

        let from_i64_var = quote! { n };
        let clauses: Vec<_> = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                match variant.fields {
                    Fields::Unit => (),
                    _ => panic!(
                        "`FromPrimitive` can be applied only to unitary enums and newtypes, \
                         {}::{} is either struct or tuple",
                        name, ident
                    ),
                }

                quote! {
                    if #from_i64_var == #name::#ident as i64 {
                        Some(#name::#ident)
                    }
                }
            }).collect();

        let from_i64_var = if clauses.is_empty() {
            quote!(_)
        } else {
            from_i64_var
        };

        quote! {
            #[allow(unused_qualifications)]
            extern crate num_traits as _num_traits;

            impl _num_traits::FromPrimitive for #name {
                #[allow(trivial_numeric_casts)]
                fn from_i64(#from_i64_var: i64) -> Option<Self> {
                    #(#clauses else)* {
                        None
                    }
                }

                fn from_u64(n: u64) -> Option<Self> {
                    Self::from_i64(n as i64)
                }
            }
        }
    };

    dummy_const_trick("FromPrimitive", &name, impl_).into()
}

/// Derives [`num_traits::ToPrimitive`][to] for simple enums and newtypes.
///
/// [to]: https://docs.rs/num-traits/0.2/num_traits/cast/trait.ToPrimitive.html
///
/// # Examples
///
/// Simple enums can be derived:
///
/// ```rust
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(ToPrimitive)]
/// enum Color {
///     Red,
///     Blue,
///     Green = 42,
/// }
/// # fn main() {}
/// ```
///
/// Enums that contain data are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(ToPrimitive)]
/// enum Color {
///     Rgb(u8, u8, u8),
///     Hsv(u8, u8, u8),
/// }
/// # fn main() {}
/// ```
///
/// Structs are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
/// #[derive(ToPrimitive)]
/// struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
/// # fn main() {}
/// ```
#[proc_macro_derive(ToPrimitive)]
pub fn to_primitive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let impl_ = if let Some(inner_ty) = newtype_inner(&ast.data) {
        let i128_fns = if cfg!(has_i128) {
            quote! {
                fn to_i128(&self) -> Option<i128> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_i128(&self.0)
                }
                fn to_u128(&self) -> Option<u128> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_u128(&self.0)
                }
            }
        } else {
            quote! {}
        };

        quote! {
            extern crate num_traits as _num_traits;
            impl _num_traits::ToPrimitive for #name {
                fn to_i64(&self) -> Option<i64> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_i64(&self.0)
                }
                fn to_u64(&self) -> Option<u64> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_u64(&self.0)
                }
                fn to_isize(&self) -> Option<isize> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_isize(&self.0)
                }
                fn to_i8(&self) -> Option<i8> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_i8(&self.0)
                }
                fn to_i16(&self) -> Option<i16> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_i16(&self.0)
                }
                fn to_i32(&self) -> Option<i32> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_i32(&self.0)
                }
                fn to_usize(&self) -> Option<usize> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_usize(&self.0)
                }
                fn to_u8(&self) -> Option<u8> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_u8(&self.0)
                }
                fn to_u16(&self) -> Option<u16> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_u16(&self.0)
                }
                fn to_u32(&self) -> Option<u32> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_u32(&self.0)
                }
                fn to_f32(&self) -> Option<f32> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_f32(&self.0)
                }
                fn to_f64(&self) -> Option<f64> {
                    <#inner_ty as _num_traits::ToPrimitive>::to_f64(&self.0)
                }
                #i128_fns
            }
        }
    } else {
        let variants = match ast.data {
            Data::Enum(ref data_enum) => &data_enum.variants,
            _ => panic!(
                "`ToPrimitive` can be applied only to enums and newtypes, {} is neither",
                name
            ),
        };

        let variants: Vec<_> = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                match variant.fields {
                    Fields::Unit => (),
                    _ => {
                        panic!("`ToPrimitive` can be applied only to unitary enums and newtypes, {}::{} is either struct or tuple", name, ident)
                    },
                }

                // NB: We have to check each variant individually, because we'll only have `&self`
                // for the input.  We can't move from that, and it might not be `Clone` or `Copy`.
                // (Otherwise we could just do `*self as i64` without a `match` at all.)
                quote!(#name::#ident => #name::#ident as i64)
            })
            .collect();

        let match_expr = if variants.is_empty() {
            // No variants found, so do not use Some to not to trigger `unreachable_code` lint
            quote! {
                match *self {}
            }
        } else {
            quote! {
                Some(match *self {
                    #(#variants,)*
                })
            }
        };

        quote! {
            #[allow(unused_qualifications)]
            extern crate num_traits as _num_traits;

            impl _num_traits::ToPrimitive for #name {
                #[allow(trivial_numeric_casts)]
                fn to_i64(&self) -> Option<i64> {
                    #match_expr
                }

                fn to_u64(&self) -> Option<u64> {
                    self.to_i64().map(|x| x as u64)
                }
            }
        }
    };

    dummy_const_trick("ToPrimitive", &name, impl_).into()
}
