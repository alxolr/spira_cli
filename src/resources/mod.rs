pub mod incident;
pub mod task;
pub mod user;

pub trait UiLink {
    fn get_link(&self) -> String;
}
