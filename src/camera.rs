use wgpu::util::DeviceExt;

use crate::{buffer::Buffer, math::Rect};

pub struct Camera {
    pos: nalgebra::Point3<f32>,
    up: nalgebra::Vector3<f32>,
    rect: Rect,
    znear: f32,
    zfar: f32,

    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    pub bg: wgpu::BindGroup,
    pub bg_layout: wgpu::BindGroupLayout,
}

impl Camera {
    fn build_view_ortho(&self) -> nalgebra::Matrix4<f32> {
        let view = nalgebra::Matrix4::look_at_rh(
            &self.pos,
            &(self.pos + nalgebra::Vector3::new(0., 0., -1.)),
            &self.up,
        );

        let proj = nalgebra::Matrix4::new_orthographic(
            self.rect.min.x,
            self.rect.max.x,
            self.rect.min.y,
            self.rect.max.y,
            self.znear,
            self.zfar,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn translate(&mut self, pos: nalgebra::Point3<f32>) {
        self.pos = pos;
        self.uniform.view_proj = self.build_view_ortho().into();
    }
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = CameraUniform::new();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bg_layout = device.create_bind_group_layout(&Self::desc());

        Self {
            pos: nalgebra::Point3::new(0., 0., 0.),
            up: nalgebra::Vector3::new(0., 0., 1.),
            rect: Rect::new(-0.5, -0.5, 1., 1.),
            znear: 0.1,
            zfar: 100.0,
            uniform,
            bg: Self::bind_group(device, &bg_layout, &buffer),
            buffer,
            bg_layout,
        }
    }

    fn desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera-bind-group-layout"),
        }
    }

    fn bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera-bind-group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }
}

impl Buffer for Camera {
    fn set_buffer<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.bg, &[]);
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: nalgebra::Matrix4::identity().into(),
        }
    }
}
