pub mod task;
pub trait UiLink {
    fn get_link(&self) -> String;
}