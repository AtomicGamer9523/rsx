//! This module contains the prelude for the rsx crate.

use crate::utils::*;
use crate::WebResult;

impl std::fmt::Debug for Dom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dom({:?})", self.0.to_string())
    }
}

impl From<axohtml::dom::DOMTree<String>> for Dom {
    fn from(dom: axohtml::dom::DOMTree<String>) -> Self {
        Self(dom)
    }
}

impl Into<WebResult> for Dom {
    fn into(self) -> WebResult {
        Ok(self.0.to_string().into())
    }
}
