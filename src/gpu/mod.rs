use bytemuck::{Pod, Zeroable};
use wgpu::{
    Adapter, Device, Instance, Queue, Surface, SwapChain, SwapChainDescriptor, TextureFormat,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EntityUniform {
    pub model: glam::Mat4,
}

unsafe impl Pod for EntityUniform {}
unsafe impl Zeroable for EntityUniform {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 3],
    //tex_coord: [f32; 2],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub local_bind_group: wgpu::BindGroup,
}

pub struct Material {
    pub pipeline: wgpu::RenderPipeline,
}

pub struct Context {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

impl Context {
    pub async fn new(window: &winit::window::Window, swap_chain_format: TextureFormat) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropiate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: swap_chain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        Context {
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
        }
    }
}
