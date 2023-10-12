use wgpu::util::DeviceExt;

pub trait Buffer {
    fn set_buffer<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct IndexBuffer<const N: usize> {
    indices: [u16; N],
    buffer: wgpu::Buffer,
}

impl<const N: usize> IndexBuffer<N> {
    pub fn new(device: &wgpu::Device, indices: [u16; N]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self { indices, buffer }
    }
}

impl<const N: usize> Buffer for IndexBuffer<N> {
    fn set_buffer<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_index_buffer(self.buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}

pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColored {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for VertexColored {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexColored>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexTextured {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl Vertex for VertexTextured {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexTextured>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct VertexBuffer<T: Vertex, const N: usize> {
    vertices: [T; N],
    buffer: wgpu::Buffer,
}

impl<T: Vertex, const N: usize> VertexBuffer<T, N> {
    pub fn new(device: &wgpu::Device, vertices: [T; N]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self { vertices, buffer }
    }
}

impl<T: Vertex, const N: usize> Buffer for VertexBuffer<T, N> {
    fn set_buffer<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
    }
}
