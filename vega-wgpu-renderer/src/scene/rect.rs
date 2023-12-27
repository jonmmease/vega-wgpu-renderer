use crate::error::VegaWgpuError;
use crate::specs::mark::MarkContainerSpec;
use crate::specs::rect::RectItemSpec;

#[derive(Debug, Clone)]
pub struct RectMark {
    pub instances: Vec<RectInstance>,
    pub clip: bool,
}

impl RectMark {
    pub fn from_spec(spec: &MarkContainerSpec<RectItemSpec>) -> Result<Self, VegaWgpuError> {
        let instances = RectInstance::from_specs(spec.items.as_slice())?;

        Ok(Self {
            instances,
            clip: spec.clip,
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub width: f32,
    pub height: f32,
}

impl RectInstance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }

    pub fn from_spec(item_spec: &RectItemSpec) -> Result<Self, VegaWgpuError> {
        // TODO: x2, y2
        let color = if let Some(fill) = &item_spec.fill {
            let c = csscolorparser::parse(fill)?;
            [c.r as f32, c.g as f32, c.b as f32]
        } else {
            [0.5f32, 0.5, 0.5]
        };

        Ok(Self {
            position: [item_spec.x, item_spec.y],
            color,
            width: item_spec.width.unwrap(),
            height: item_spec.height.unwrap(),
        })
    }

    pub fn from_specs(item_specs: &[RectItemSpec]) -> Result<Vec<Self>, VegaWgpuError> {
        item_specs
            .iter()
            .map(Self::from_spec)
            .collect::<Result<Vec<_>, VegaWgpuError>>()
    }
}