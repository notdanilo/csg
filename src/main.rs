pub mod exporter;
pub mod mesh;
pub mod scene;
pub mod camera;
pub mod field;

use exporter::Exporter;
use mesh::{Mesh, Rasterizer};
use camera::Camera;
use scene::Scene;
use field::Field3;

use nalgebra::Vector3;
pub type Scalar = f32;
pub type VectorField3 = Field3<Vector3<Scalar>>;
pub type ScalarField3 = Field3<Scalar>;

fn main() {
    let voxel_grid = ScalarField3::new();
    let exporter = Exporter::new();

    let size = (800, 600);
    let mut window = gpu::Window::new("Constructive Solid Geometry".to_string(), size);
    window.on_resize(Some(|resize: gpu::OnResizeEvent| {
        // println!("{:?} x {:?}", resize.previous_size, resize.size);
    }));
    let display = gpu::ContextDisplay::Window(window);
    let mut renderer = Rasterizer::new(display);
    let mut camera = Camera::default(renderer.context());
    let mesh = Mesh::generate(renderer.context());
    exporter.export(&mesh, "mesh.stl");

    while renderer.run() {
        renderer.raster(&mesh, &mut camera);
    }
}
