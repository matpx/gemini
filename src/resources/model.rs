use super::{manager::ResourceManager, prefab::Prefab, scene::Scene, LoaderError};
use crate::{
    components::{MeshComponent, MeshPrimitive, TransformComponent},
    gpu::{Context, Geometry, Texture, Vertex},
};
use gltf::{Node, Primitive};
use itertools::izip;
use slotmap::DefaultKey;
use std::collections::{hash_map::Entry, HashMap};
use wgpu::TextureFormat;

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

    let mut out_data = Vec::with_capacity((data.len() / from_size) * to_size);

    for chunk in data.chunks(from_size) {
        out_data.extend_from_slice(chunk);

        for _ in 0..to_size - from_size {
            out_data.push(255);
        }
    }

    out_data
}

fn load_primitive_textures(
    context: &Context,
    images: &[gltf::image::Data],
    primitive: &Primitive,
) -> Option<Texture> {
    if let Some(info) = primitive
        .material()
        .pbr_metallic_roughness()
        .base_color_texture()
    {
        let image_data = &images[info.texture().index()];

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
                TextureFormat::Bgra8Unorm,
            ),
            gltf::image::Format::R16G16B16A16 => (&image_data.pixels, TextureFormat::Rgba16Uint),
        };

        let texture = Texture::new(
            &context.device,
            &context.queue,
            &context.uniform_layouts,
            (image_data.width, image_data.height),
            format,
            &pixels,
        );

        Some(texture)
    } else {
        None
    }
}

fn load_node(
    context: &Context,
    resource_manager: &mut ResourceManager,
    scene: &mut Scene,
    node: &Node,
    buffers: &[gltf::buffer::Data],
    images: &[gltf::image::Data],
    parent: Option<DefaultKey>,
    known_meshes: &mut HashMap<usize, DefaultKey>,
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
        let mesh_component =
            match known_meshes.entry(mesh.index()) {
                Entry::Occupied(v) => {
                    let source_mesh_component = scene.meshes.get(*v.get()).unwrap();

                    let mut mc = MeshComponent::new();

                    for ref_prim in &source_mesh_component.primitives {
                        mc.primitives.push(MeshPrimitive {
                            geometry_id: ref_prim.geometry_id,
                            pipeline_id: ref_prim.pipeline_id,
                            color_texture: ref_prim.color_texture,
                        });
                    }

                    mc
                }
                Entry::Vacant(v) => {
                    let mut mc = MeshComponent::new();

                    for gltf_primitive in mesh.primitives() {
                        let geometry_id = resource_manager
                            .geometries
                            .insert(load_primitive_geometry(context, buffers, &gltf_primitive)?);

                        let texture_id = if let Some(texture) =
                            load_primitive_textures(context, images, &gltf_primitive)
                        {
                            Some(resource_manager.texture.insert(texture))
                        } else {
                            None
                        };

                        let mesh_primitive = MeshPrimitive {
                            geometry_id,
                            pipeline_id: 0,
                            color_texture: texture_id,
                        };

                        mc.primitives.push(mesh_primitive);
                    }

                    v.insert(entity);
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
            buffers,
            images,
            Some(entity),
            known_meshes,
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

    let mut prefab = Prefab::default();
    prefab.root = prefab.scene.create_entity(TransformComponent::default());

    let mut known_meshes = HashMap::new();

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(
                context,
                resource_manager,
                &mut prefab.scene,
                &node,
                &buffers,
                &images,
                Some(prefab.root),
                &mut known_meshes,
            )?;
        }
    }

    Ok(prefab)
}
