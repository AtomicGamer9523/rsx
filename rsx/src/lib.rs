#![deny(absolute_paths_not_starting_with_crate)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::all)]

//! # RSX
//! a web framework for Rust made to feel like ReactJS

extern crate rsx_macros;
mod webserver;

pub use rsx_macros::*;
pub use webserver::*;

/// The DOM type used by RSX
pub type Dom = axohtml::dom::DOMTree<String>;

/// The result wrapper for RSX
pub mod res;

/// The HTML module
pub mod html {
    #[doc(hidden)]
    pub use axohtml::html as __html__;
    
    pub use axohtml::{
        dom,
        elements, events, escape_html_attribute,
        OutputType,
        types,
        unsafe_text
    };
}
