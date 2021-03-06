use super::{manager::ResourceManager, prefab::Prefab, scene::Scene, LoaderError};
use crate::{
    components::{material::PbrMaterial, MeshComponent, MeshPrimitive, TransformComponent},
    gpu::{Context, Geometry, Texture, Vertex},
};
use gltf::{buffer, image, Node, Primitive};
use itertools::izip;
use slotmap::DefaultKey;
use std::collections::{hash_map::Entry, HashMap};
use wgpu::TextureFormat;

#[derive(Debug)]
struct GltfData {
    buffers: Vec<buffer::Data>,
    images: Vec<image::Data>,
}

#[derive(Debug, Default)]
struct CacheData {
    known_meshes: HashMap<usize, MeshComponent>,
    known_textures: HashMap<usize, usize>,
}

fn load_primitive_geometry(
    context: &Context,
    buffers: &[gltf::buffer::Data],
    primitive: &Primitive,
) -> Result<Geometry, Box<dyn std::error::Error>> {
    let mut index_data: Vec<u32> = Vec::new();
    let mut vertex_data: Vec<Vertex> = Vec::new();

    let reader = primitive.reader(|b| Some(&buffers[b.index()]));

    let index_data_iter = reader.read_indices().ok_or(LoaderError)?;
    let vertex_data_iter = reader.read_positions().ok_or(LoaderError)?;
    let uv_data_iter = reader.read_tex_coords(0).ok_or(LoaderError {})?;
    let normal_data_iter = reader.read_normals().ok_or(LoaderError {})?;

    for index in index_data_iter.into_u32() {
        index_data.push(index);
    }

    for (position, uv, normal) in izip!(vertex_data_iter, uv_data_iter.into_f32(), normal_data_iter)
    {
        vertex_data.push(Vertex {
            position,
            uv,
            normal,
        });
    }

    Ok(Geometry::new(&context.device, &vertex_data, &index_data))
}

fn extend_data_color(data: &[u8], from_size: usize, to_size: usize) -> Vec<u8> {
    assert!(to_size > from_size);

    let padding_values = vec![255; to_size - from_size];

    let mut out_data = Vec::with_capacity((data.len() / from_size) * to_size);

    for chunk in data.chunks(from_size) {
        out_data.extend_from_slice(chunk);

        out_data.extend(&padding_values);
    }

    out_data
}

fn load_primitive_textures(
    context: &Context,
    images: &[gltf::image::Data],
    source_index: usize,
) -> Texture {
    let image_data = &images[source_index];

    let copy_buffer;

    let (pixels, format) = match image_data.format {
        gltf::image::Format::R8 => (&image_data.pixels, TextureFormat::R8Unorm),
        gltf::image::Format::R8G8 => (&image_data.pixels, TextureFormat::Rg8Unorm),
        gltf::image::Format::R8G8B8 => (
            {
                copy_buffer = extend_data_color(&image_data.pixels, 3, 4);
                &copy_buffer
            },
            TextureFormat::Rgba8Unorm,
        ),
        gltf::image::Format::R8G8B8A8 => (&image_data.pixels, TextureFormat::Rgba8Unorm),
        gltf::image::Format::B8G8R8 => (
            {
                copy_buffer = extend_data_color(&image_data.pixels, 3, 4);
                &copy_buffer
            },
            TextureFormat::Bgra8Unorm,
        ),
        gltf::image::Format::B8G8R8A8 => (&image_data.pixels, TextureFormat::Bgra8Unorm),
        gltf::image::Format::R16 => (&image_data.pixels, TextureFormat::R16Uint),
        gltf::image::Format::R16G16 => (&image_data.pixels, TextureFormat::Rg16Uint),
        gltf::image::Format::R16G16B16 => (
            {
                copy_buffer = extend_data_color(&image_data.pixels, 6, 8);
                &copy_buffer
            },
            TextureFormat::Rgba16Uint,
        ),
        gltf::image::Format::R16G16B16A16 => (&image_data.pixels, TextureFormat::Rgba16Uint),
    };

    Texture::new(
        &context.device,
        &context.queue,
        &context.uniform_layouts,
        (image_data.width, image_data.height),
        format,
        &pixels,
    )
}

fn load_primtive(
    context: &Context,
    resource_manager: &mut ResourceManager,
    gltf_data: &GltfData,
    gltf_primitive: &Primitive,
    known_textures: &mut HashMap<usize, usize>,
) -> Result<MeshPrimitive, Box<dyn std::error::Error>> {
    let geometry_id = resource_manager.geometries.insert(load_primitive_geometry(
        context,
        &gltf_data.buffers,
        &gltf_primitive,
    )?);

    let gltf_pbr_material = gltf_primitive.material().pbr_metallic_roughness();

    let color_texture = if let Some(info) = gltf_pbr_material.base_color_texture() {
        let source_index = info.texture().source().index();

        Some(*known_textures.entry(source_index).or_insert_with(|| {
            let texture = load_primitive_textures(context, &gltf_data.images, source_index);

            resource_manager.texture.insert(texture)
        }))
    } else {
        None
    };

    let material = PbrMaterial {
        color: gltf_pbr_material.base_color_factor().into(),
        color_texture,
    };

    Ok(MeshPrimitive {
        geometry_id,
        pipeline_id: 0,
        material,
    })
}

fn load_node(
    context: &Context,
    resource_manager: &mut ResourceManager,
    scene: &mut Scene,
    node: &Node,
    gltf_data: &GltfData,
    parent: Option<DefaultKey>,
    cache: &mut CacheData,
) -> Result<DefaultKey, Box<dyn std::error::Error>> {
    let gltf_transform = node.transform().decomposed();
    let transform = TransformComponent {
        translation: gltf_transform.0.into(),
        rotation: gltf_transform.1.into(),
        scale: gltf_transform.2.into(),
        parent,
        ..Default::default()
    };

    let entity = scene.create_entity(transform);
    if let Some(mesh) = node.mesh() {
        let mesh_component = match cache.known_meshes.entry(mesh.index()) {
            Entry::Occupied(v) => v.get().clone(),
            Entry::Vacant(v) => {
                let mut mc = MeshComponent::new();

                for gltf_primitive in mesh.primitives() {
                    mc.primitives.push(load_primtive(
                        context,
                        resource_manager,
                        gltf_data,
                        &gltf_primitive,
                        &mut cache.known_textures,
                    )?);
                }

                v.insert(mc.clone());

                mc
            }
        };

        scene.meshes.insert(entity, mesh_component);
    }

    for child_node in node.children() {
        load_node(
            context,
            resource_manager,
            scene,
            &child_node,
            gltf_data,
            Some(entity),
            cache,
        )?;
    }

    Ok(entity)
}

pub fn load_gltf(
    context: &Context,
    resource_manager: &mut ResourceManager,
    path: &str,
) -> Result<Prefab, Box<dyn std::error::Error>> {
    let (document, buffers, images) = gltf::import(path)?;
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    let gltf_data = GltfData { buffers, images };

    let mut prefab = Prefab::default();
    prefab.root = prefab.scene.create_entity(TransformComponent::default());

    let mut cache = CacheData::default();

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(
                context,
                resource_manager,
                &mut prefab.scene,
                &node,
                &gltf_data,
                Some(prefab.root),
                &mut cache,
            )?;
        }
    }

    Ok(prefab)
}
