#![deny(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::all)]

//! # RSX Macros
//! a web framework for Rust made to feel like ReactJS

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// Creates a renderable page
/// 
/// This macro is used to create a renderable page. It is used in conjunction with the `rsx!` macro. \
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
pub fn render() -> rsx::Dom {{ {} }}",item);
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
    let mut output: TokenStream = (quote! {
        #[::actix_web::rt::main(system = "::actix_web::rt::System")]
    })
    .into();

    output.extend(item);
    output
}

/// Construct a DOM tree.
///
/// 
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let res = format!("rsx::html::__html__!{{{}}}",input);
    match res.parse() {
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}
