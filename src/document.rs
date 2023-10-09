pub struct Document<'a> {
    texture_size: wgpu::Extent3d,
    buffers: Vec<ImageBuffer<'a>>,
}

impl Document<'_> {
    fn new(width: u32, height: u32) -> Self {
        Self {
            texture_size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            buffers: Vec::new(),
        }
    }
}

pub struct ImageBuffer<'a> {
    texture_size: &'a wgpu::Extent3d,
    layers: Vec<Layer<'a>>,
}

impl<'a> ImageBuffer<'a> {
    pub fn new(texture_size: &'a wgpu::Extent3d) -> Self {
        Self {
            texture_size,
            layers: Vec::new(),
        }
    }
}

pub struct Layer<'a> {
    texture_size: &'a wgpu::Extent3d,
    diffuse_texture: wgpu::Texture,
}
