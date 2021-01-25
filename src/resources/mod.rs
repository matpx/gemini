mod model;

pub use model::load_gltf;

#[derive(Debug, Clone)]
pub struct LoaderError;

impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to load resource")
    }
}

impl std::error::Error for LoaderError {}
