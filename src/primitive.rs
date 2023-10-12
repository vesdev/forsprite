use crate::math::Rect;

use crate::buffer::{Buffer, IndexBuffer, Vertex, VertexBuffer, VertexTextured};

pub trait Draw {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}

// //draw stuff

pub struct Triangle<T: Vertex> {
    vertices: VertexBuffer<T, 3>,
}

impl<T: Vertex> Draw for Triangle<T> {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.draw(0..3, 0..1); // 3.
    }
}

impl<T: Vertex> Triangle<T> {
    pub fn new(_device: &wgpu::Device, vertices: VertexBuffer<T, 3>) -> Self {
        Self { vertices }
    }
}

pub struct Quad<T: Vertex> {
    vertices: VertexBuffer<T, 4>,
    indices: IndexBuffer<6>,
}

impl<T: Vertex> Draw for Quad<T> {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertices.set_buffer(render_pass);
        self.indices.set_buffer(render_pass);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}

impl<T: Vertex> Quad<T> {
    /// Creates a quad with vertices defined in the order
    /// bottom left
    /// bottom right
    /// top left
    /// top right

    /// 2------3
    /// |    / |
    /// |   /  |
    /// |  /   |
    /// | /    |
    /// 0------1
    pub fn new(device: &wgpu::Device, vertices: VertexBuffer<T, 4>) -> Self {
        let indices = IndexBuffer::new(device, [0, 3, 2, 0, 1, 3]);
        Self { vertices, indices }
    }
}

pub struct Image {
    quad: Quad<VertexTextured>,
    // view: wgpu::TextureView,
    // sampler: wgpu::Sampler,
    pub bg: wgpu::BindGroup,
    pub bg_layout: wgpu::BindGroupLayout,
}

impl Draw for Image {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bg, &[]);
        self.quad.draw(render_pass);
    }
}

impl Image {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
        rect: Rect,
    ) -> Self {
        let vertices = VertexBuffer::new(
            device,
            [
                // 0
                VertexTextured {
                    position: [rect.min.x, rect.min.y, 0.0],
                    tex_coord: [0.0, 1.0],
                },
                //1
                VertexTextured {
                    position: [rect.max.x, rect.min.y, 0.0],
                    tex_coord: [1.0, 1.0],
                },
                // 2
                VertexTextured {
                    position: [rect.min.x, rect.max.y, 0.0],
                    tex_coord: [0.0, 0.0],
                },
                // 3
                VertexTextured {
                    position: [rect.max.x, rect.max.y, 0.0],
                    tex_coord: [1.0, 0.0],
                },
            ],
        );

        let image = image::load_from_memory(bytes).unwrap();
        let desc = Self::desc();
        let layout = device.create_bind_group_layout(&desc);

        Self {
            quad: Quad::new(device, vertices),
            bg: Self::bind_group(device, queue, &layout, &image, label),
            bg_layout: layout,
        }
    }

    fn desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("image-bind-group-layout"),
        }
    }

    fn bind_group(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        image: &image::DynamicImage,
        label: &str,
    ) -> wgpu::BindGroup {
        let diffuse_rgba = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(&format!("{}-texture", label)),
            view_formats: &[],
        });
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some(&format!("{}-bind-group", label)),
        })
    }
}
