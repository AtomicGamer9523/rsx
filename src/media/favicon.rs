use crate::media::MediaServable;

/// Allows you to serve a favicon
#[derive(Debug)]
pub struct Favicon {
    
}

impl MediaServable for Favicon {
    fn serve(&self) -> crate::WebResult {
        todo!()
    }
}

impl Favicon {
    /// TODO
    pub fn new<T: ToString>(path: T) -> Self {
        todo!()
    }
}

