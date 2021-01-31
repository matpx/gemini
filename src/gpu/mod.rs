mod context;
mod geometry;
mod pipeline;
mod render;
pub mod uniform;
mod vertex;

pub use context::Context;
pub use geometry::Geometry;
pub use pipeline::Pipeline;
pub use render::render;
pub use vertex::Vertex;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[cfg(not(target_arch = "wasm32"))]
pub const SWAPCHAIN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

#[cfg(target_arch = "wasm32")]
pub const SWAPCHAIN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
