#![deny(absolute_paths_not_starting_with_crate)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::all)]

#![feature(associated_type_defaults)]

//! # RSX
//! a web framework for Rust made to feel like ReactJS

mod utils;
mod prelude; pub use prelude::*;
extern crate rsx_macros;

pub use utils::*;
pub use rsx_macros::{
    page,
    main,
    rsx
};

pub use tide::Result as WebResult;

pub(crate) mod consts;
pub(crate) mod favicon;

/// The result wrapper for RSX
pub mod res;

/// The HTML module
pub mod html {
    pub use axohtml::{
        dom,
        elements, events, escape_html_attribute,
        OutputType,
        types,
        unsafe_text
    };
    pub use crate::favicon::Favicon;
}

/// The routing module
#[macro_export(local_inner_macros)]
macro_rules! route {
    ($name:literal <= $path:literal) => {
        rsx::__internals__::__route__!($name <= $path);
    };
    ($name:literal) => {
        rsx::__internals__::__route__!($name <= ".");
    };
    () => ()
}

#[doc(hidden)]
pub mod __internals__ {
    pub use axohtml::html as __html__;
    pub use rsx_macros::__route__;
    pub use async_std as __async_std__;
}

/// Returns a new tide server
#[allow(non_snake_case)]
pub fn newApp() -> tide::Server<()> {
    tide::new()
}
