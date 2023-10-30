use std::env;
use std::ffi::CString;

use assimp_sys::*;
use black_hole::models::models::{TextureVertex, Mesh, Model};
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
        
        println!("Processing file: {0}", &args[1]);

        if !scene.is_null() {
            let meshes = process_node((*scene).root_node, scene);

            let model = Model::new(args[2].clone(), meshes);

            println!("Serializing to file: {0}", &args[2]);

            model.serialize_to_file(&args[2]);
        }
        else {
            println!("Failed to load AiScene for file: {0}", &args[1]);
        }
    }
}

fn are_args_valid(args: &Vec<String>) -> bool {
    if args.len() != 3 {
        println!("Usage: {0} <input_file> <output_file>", &args[0]);
        return false;
    }

    true
}

unsafe fn process_node(node: *mut AiNode, scene: *const AiScene) -> Vec<Mesh> {
    println!("Processing Node");

    let mut to_return = Vec::new();
    for i in 0..(*node).num_meshes as isize {
        let mesh_index = (*(*node).meshes.offset(i)) as isize;
        println!("index: {0}, mesh index: {1}, num meshes: {2}, num node meshes: {3}", i, mesh_index, (*scene).num_meshes, (*node).num_meshes);
        let ai_mesh = (*(*scene).meshes).offset(mesh_index);
        if !ai_mesh.is_null() {
            to_return.push(process_mesh(ai_mesh));
        }
        else {
            println!("Empty Mesh");
        }
    }
    
    println!("Processing Child Nodes");
    for i in 0..(*node).num_children as isize {
        to_return.append(&mut process_node(*(*node).children.offset(i), scene));
    }

    to_return
}

unsafe fn process_mesh(mesh: *const AiMesh) -> Mesh {
    println!("Processing Mesh");

    let mut vertices = Vec::new();
    for i in 0..(*mesh).num_vertices as isize {
        println!("Processing Position");
        let position = Vec3 { x: (*(*mesh).vertices.offset(i)).x, y: (*(*mesh).vertices.offset(i)).y, z: (*(*mesh).vertices.offset(i)).z };
        println!("Processing TexturePos");
        let tex_position = Vec2 { x: (*(*mesh).texture_coords[0].offset(i)).x, y: (*(*mesh).texture_coords[0].offset(i)).y };
        vertices.push(TextureVertex { position, tex_position });
    }

    println!("Creating Mesh");
    let mut indices = Vec::new();
    let mut textures = Vec::new();
    Mesh::new(vertices, indices, textures)
}
