use super::{
    uniform::{UniformContext, UniformLayouts},
    DEPTH_FORMAT, SWAPCHAIN_FORMAT,
};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SwapChain, SwapChainDescriptor};
use winit::dpi::PhysicalSize;

pub struct Context {
    size: PhysicalSize<u32>,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
    pub uniforms: UniformContext,
    pub uniform_layouts: UniformLayouts,
}

impl Context {
    pub async fn new(window: &winit::window::Window) -> Self {
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
            format: SWAPCHAIN_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let uniform_layouts = UniformLayouts::new(&device);

        let uniforms = UniformContext::new(&device, &uniform_layouts, &size);

        Context {
            size,
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            uniforms,
            uniform_layouts,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.swap_chain_desc.width = size.width;
        self.swap_chain_desc.height = size.height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        });

        self.uniforms.depth_view =
            depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
}
