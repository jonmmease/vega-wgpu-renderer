use crate::renderers::mark::MarkShader;
use crate::renderers::vertex::Vertex;
use crate::scene::symbol::SymbolInstance;

pub struct SymbolShader {
    verts: Vec<Vertex>,
    indices: Vec<u16>,
    shader: String,
    vertex_entry_point: String,
    fragment_entry_point: String,
}

impl SymbolShader {
    pub fn new() -> Self {
        let tan30: f32 = (30.0 * std::f32::consts::PI / 180.0).tan();
        let radius: f32 = (1.0 / (2.0 * tan30)).sqrt();

        Self {
            verts: vec![
                Vertex {
                    position: [0.0, -radius, 0.0],
                },
                Vertex {
                    position: [radius, 0.0, 0.0],
                },
                Vertex {
                    position: [0.0, radius, 0.0],
                },
                Vertex {
                    position: [-radius, 0.0, 0.0],
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            shader: include_str!("symbol.wgsl").to_string(),
            vertex_entry_point: "vs_main".to_string(),
            fragment_entry_point: "fs_main".to_string(),
        }
    }
}

impl MarkShader for SymbolShader {
    type Instance = SymbolInstance;

    fn verts(&self) -> &[Vertex] {
        self.verts.as_slice()
    }

    fn indices(&self) -> &[u16] {
        self.indices.as_slice()
    }

    fn shader(&self) -> &str {
        self.shader.as_str()
    }

    fn vertex_entry_point(&self) -> &str {
        self.vertex_entry_point.as_str()
    }

    fn fragment_entry_point(&self) -> &str {
        self.fragment_entry_point.as_str()
    }

    fn instance_desc(&self) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SymbolInstance>() as wgpu::BufferAddress,
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
            ],
        }
    }
}
