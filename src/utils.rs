//! HTTP-Based utilities for RSX

/// The DOM type used by RSX
pub struct Dom(pub(crate) axohtml::dom::DOMTree<String>);
