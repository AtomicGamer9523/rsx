pub(crate) const SERVER: &str = concat!(
    "HyperNet/",
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION_MAJOR"),
    ".",
    env!("CARGO_PKG_VERSION_MINOR")
);