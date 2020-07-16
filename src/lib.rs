use gtk;

pub trait Gladis {
    fn from_builder(builder: gtk::Builder) -> Self;
    fn from_string(src: &str) -> Self;
    fn from_resource(resource_path: &str) -> Self;
}