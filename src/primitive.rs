use wgpu::util::DeviceExt;
// lib.rs
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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

#[derive(Clone)]
pub struct Vertices<const N: usize> {
    vertices: [Vertex; N],
}

impl<const N: usize> Vertices<N> {
    pub fn to_buffer(&self, device: &mut wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

impl<const N: usize> From<[Vertex; N]> for Vertices<N> {
    fn from(value: [Vertex; N]) -> Self {
        Self { vertices: value }
    }
}

pub trait Shape {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}
// //draw stuff

pub struct Triangle {
    vertices: Vertices<3>,
    buffer: wgpu::Buffer,
}

impl Shape for Triangle {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.draw(0..3, 0..1); // 3.
    }
}

impl Triangle {
    pub fn new(device: &mut wgpu::Device, vertices: Vertices<3>) -> Self {
        Self {
            buffer: vertices.to_buffer(device),
            vertices,
        }
    }
}
