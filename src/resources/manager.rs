use crate::gpu::{Geometry, Pipeline};
use slab::Slab;

#[derive(Default)]
pub struct ResourceManager {
    pub geometries: Slab<Geometry>,
    pub pipelines: Slab<Pipeline>,
}
