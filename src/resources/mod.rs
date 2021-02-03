pub mod manager;
pub mod map;
pub mod model;
pub mod prefab;
pub mod scene;

#[derive(Debug, Clone)]
pub struct LoaderError;

impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to load resource")
    }
}

impl std::error::Error for LoaderError {}
