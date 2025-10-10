pub trait FileOps {
    fn list_file_path(path: &str) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>>;
}

#[cfg(feature = "derive")]
pub use file_macro_derive::*;
