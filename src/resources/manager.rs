use crate::gpu::{Geometry, Pipeline, Texture};
use slab::Slab;

#[derive(Default)]
pub struct ResourceManager {
    pub geometries: Slab<Geometry>,
    pub pipelines: Slab<Pipeline>,
    pub texture: Slab<Texture>,
}
