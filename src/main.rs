use std::env;
use std::ffi::CString;

use assimp_sys::*;
use black_hole::models::{TextureVertex, Mesh};
use glam::{Vec2, Vec3};

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();

        if !are_args_valid(&args) {
            return;
        }

        println!("Loading file: {0}", &args[1]);

        let file_name = match CString::new(args[1].as_str()) {
            Ok(string) => string,
            Err(_) => return
        };

        let scene = aiImportFile(file_name.as_ptr(), AiPostProcessSteps::empty());

        
    }
}

fn are_args_valid(args: &Vec<String>) -> bool {
    if args.len() != 2 {
        println!("Usage: {0} <input_file>", &args[0]);
        return false;
    }

    true
}

fn process_node(node: &AiNode, scene: &AiScene) -> Vec<Mesh> {
    let to_return = Vec::new();
    for i in 0..node.num_meshes as isize {
        let ai_mesh = **scene.meshes.offset(*node.meshes.offset(i) as isize);
        to_return.push(process_mesh(&ai_mesh));
    }

    for i in 0..node.num_children as isize {
        to_return.append(process_node(&**node.children.offset(i), scene));
    }

    to_return
}

fn process_mesh(mesh: &AiMesh) -> Mesh {
    let vertices = Vec::new();
    for i in 0..mesh.num_vertices as isize {
        let position = Vec3 { x: (*mesh.vertices.offset(i)).x, y: (*mesh.vertices.offset(i)).y, z: (*mesh.vertices.offset(i)).z };
        let tex_position = Vec2 { x: *(mesh.texture_coords.offset(0)[i]).x, y: *(mesh.texture_coords.offset(0)[i]).y };
        vertices.push(TextureVertex { position, tex_position });
    }

    let indices = Vec::new();
    let textures = Vec::new();
    Mesh::new(&vertices, &indices, &textures)
}
