mod context;
mod geometry;
mod pipeline;
mod render;
mod texture;
pub mod uniform;
mod vertex;

pub use context::Context;
pub use geometry::Geometry;
pub use pipeline::Pipeline;
pub use render::render;
pub use texture::Texture;
pub use vertex::Vertex;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
