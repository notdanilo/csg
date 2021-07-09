use crate::camera::Camera;
use crate::scene::Scene;
use crate::mesh::Mesh;
use gpu::RasterProgram;

pub struct Rasterizer {
    context: gpu::Context,
    clear_program: gpu::ClearProgram,
    raster_program: gpu::RasterProgram
}

impl Rasterizer {
    pub fn new(display: gpu::ContextDisplay) -> Self {
        let context = gpu::ContextBuilder::new().with_display(display).build();
        context.make_current().expect("Couldn't make current.");

        let clear_program = gpu::ClearProgram::new(&context);
        let vertex_shader = gpu::VertexShader::new(&context, include_str!("vertex.glsl")).expect("Couldn't create VertexShader.");
        let fragment_shader = gpu::FragmentShader::new(&context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
        let raster_program = gpu::RasterProgram::new(&context, &vertex_shader, &fragment_shader).expect("Couldn't create RasterProgram.");
        Self { context, clear_program, raster_program }
    }

    pub fn context(&self) -> &gpu::Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut gpu::Context {
        &mut self.context
    }

    pub fn run(&mut self) -> bool {
        self.context.run()
    }

    pub fn raster(&mut self, mesh: &Mesh, camera: &mut Camera) {
        self.clear_program.clear(camera.framebuffer(), gpu::ClearProgram::COLOR);
        self.raster_program.raster(camera.framebuffer(), mesh.vertex_array_object(), gpu::RasterGeometry::Triangles, mesh.vertices_number());
        self.context.swap_buffers().ok();
    }
}