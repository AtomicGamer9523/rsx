#![deny(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::all)]

//! # RSX Macros
//! a web framework for Rust made to feel like ReactJS

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Creates a renderable page
/// 
/// This macro is used to create a renderable page. It is used in conjunction with the [`rsx!`](`macro@rsx`) macro. \
/// Example Usage:
/// ```rust
/// use rsx::*;
/// 
/// #[page]
/// rsx! {
///     <html>
///         <head>
///             <title>"Hello World"</title>
///         </head>
///         <body>
///             <h1>"Hello World"</h1>
///         </body>
///     </html>
/// }
/// ```
#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let res = format!("#[doc(hidden)]mod axohtml{{pub use rsx::html::*;}}use rsx::html::*;
pub fn render() -> rsx::Dom {{ let res: rsx::html::dom::DOMTree<String> = {}; rsx::Dom::from(res) }}",item);
    match res.parse() {
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}

/// Serves Media
#[proc_macro_attribute]
pub fn media(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let res = format!("#[doc(hidden)]mod axohtml{{pub use rsx::html::*;}}use rsx::html::*;
pub fn render() -> rsx::Dom {{ let res: rsx::html::dom::DOMTree<String> = {}; rsx::Dom::from(res) }}",item);
    match res.parse() {
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}


/// Async runner for RSX
/// 
/// # Examples
/// ```rust
/// #[rsx::main]
/// async fn main() {
///     async {
///         println!("Hello world");
///     }.await
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[rsx::main]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            rsx::__internals__::__async_std__::task::block_on(async {
                main().await
            })
        }

    };

    result.into()
}

/// Construct a DOM tree.
/// 
/// This macro is used to construct an html DOM, use with the [`#[page]`](`macro@page`) macro. \
/// Example Usage:
/// ```rust,no_run
/// use rsx::*;
/// 
/// #[page]
/// rsx! {
///     <html>
///         <head>
///             <title>"Hello World"</title>
///         </head>
///         <body>
///             <h1>"Hello World"</h1>
///         </body>
///     </html>
/// }
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let res = format!("rsx::__internals__::__html__!{{{}}}",input);
    match res.parse() {
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}

/// DO NOT USE DIRECTLY
#[doc(hidden)]
#[proc_macro]
pub fn __route__(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let mut name = "app".to_string();
    let mut path = ".".to_string();
    let mut index = 0;
    for string in item.split("<=") {
        let string = string.trim();
        if index == 0 {
            name = string.replace("\"","");
        } else if index == 1 {
            path = string.replace("\"","");
        } 
        index+=1;
    }
    let res = format!("/// @IMPORTED BY RSX
    #[path = \"{}/{}.rsx\"]
    pub mod {};",path, name, name);
    match res.parse() {
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}
