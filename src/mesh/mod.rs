mod rasterizer;
pub use rasterizer::Rasterizer;

pub struct Mesh {
    vertex_array_object: gpu::VertexArrayObject,
    positions: gpu::Buffer,
    normals: gpu::Buffer,
    vertices_number: usize
}

impl Mesh {
    pub fn generate(context: &gpu::Context) -> Self {
        let positions_data: Vec<f32> = vec![
            -1.0, 0.0, 0.0,
            1.0,  0.0, 0.0,
            0.0,  1.0, 0.0
        ];

        let normals_data: Vec<f32> = vec![
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0
        ];

        let positions = gpu::Buffer::from_data(context, &positions_data);
        let normals   = gpu::Buffer::from_data(context, &normals_data);

        let mut vertex_array_object = gpu::VertexArrayObject::new(context);
        vertex_array_object.set_vertex_buffer(&positions, 0, 3);
        vertex_array_object.set_vertex_buffer(&normals, 1, 3);
        let vertices_number = 3;
        Self { vertex_array_object, vertices_number, positions, normals }
    }

    pub fn vertices_number(&self) -> usize {
        self.vertices_number
    }

    pub fn vertex_array_object(&self) -> &gpu::VertexArrayObject {
        &self.vertex_array_object
    }

    pub fn positions(&self) -> &gpu::Buffer {
        &self.positions
    }

    pub fn normals(&self) -> &gpu::Buffer {
        &self.normals
    }
}