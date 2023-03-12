pub(crate) mod favicon;

pub trait MediaServable {
    fn serve(&self) -> crate::WebResult;
}
