use crate::mesh::Mesh;
use std::fs::OpenOptions;
use stl_io::{Normal, Vertex};

pub struct Exporter {

}

impl Exporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn export(&self, mesh: &Mesh, path: &str) {
        let mut stl_mesh: Vec<stl_io::Triangle> = Vec::new();
        let positions: Vec<f32> = mesh.positions().data();
        let normals: Vec<f32> = mesh.normals().data();
        let triangles = mesh.vertices_number() / 3;
        for index in 0..triangles {
            let triangle_index = index * 3 * 3;
            let mut vertex_index = triangle_index;
            stl_mesh.push(stl_io::Triangle {
                normal: Normal::new([normals[triangle_index], normals[triangle_index + 1], normals[triangle_index + 2]]),
                vertices: [
                    Vertex::new([positions[vertex_index + 0], positions[vertex_index + 1], positions[vertex_index + 2]]),
                    Vertex::new([positions[vertex_index + 3], positions[vertex_index + 4], positions[vertex_index + 5]]),
                    Vertex::new([positions[vertex_index + 6], positions[vertex_index + 7], positions[vertex_index + 8]])
                ]
            })
        }
        let mut file = OpenOptions::new().write(true).create(true).open(path).unwrap();
        stl_io::write_stl(&mut file, stl_mesh.iter()).unwrap();
    }
}
