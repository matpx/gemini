mod context;
mod data;
mod layouts;

pub use context::UniformContext;
pub use data::CameraUniformData;
pub use data::PrimitiveUniformData;
pub use data::TransformUniformData;
pub use layouts::UniformLayouts;

pub const BUFFER_ENTITIES_NUM: u64 = 32;
