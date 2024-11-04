use std::{
    mem::zeroed,
    path,
    ptr::{null, null_mut},
    slice,
    str::FromStr,
    string,
};

use bytemuck::fill_zeroes;
use glam::{Vec2, Vec3};

use crate::{
    assimp::{
        aiGetMaterialTexture, aiGetMaterialTextureCount, aiMaterial, aiMesh, aiNode, aiScene,
        aiString, aiTextureType, aiVector2D, aiVector3D, import_file, j0,
    }, gl::Gl, mesh::{Mesh, Texture, Vertex}, renderer::texture::TextureManager, shader::{DrawableShader, Shader}
};

pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn new(gl: &Gl, path: &str) -> Self {
        let mut model = Self {
            meshes: vec![],
            directory: String::new(),
        };
        model.load_model(gl, path);
        model
    }
    pub fn draw(&self, gl: &Gl, shader: &dyn DrawableShader) {
        for mesh in self.meshes.as_slice() {
            mesh.draw(gl, shader);
        }
    }

    fn load_model(&mut self, gl: &Gl, path: &str) {
        let scene = import_file(path);
        self.directory = String::from_str(
            path::PathBuf::from_str(path)
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .unwrap();

        unsafe {
            self.process_node(gl, scene.mRootNode.as_ref().unwrap(), scene);
        }
    }
    fn process_node(&mut self, gl: &Gl, node: &aiNode, scene: &aiScene) {
        let mesh_indices = unsafe { slice::from_raw_parts(node.mMeshes, node.mNumMeshes as usize) };
        let meshes = unsafe { slice::from_raw_parts(scene.mMeshes, scene.mNumMeshes as usize) };
        for mesh_index in mesh_indices {
            let mesh = meshes[*mesh_index as usize];
            self.meshes
                .push(self.process_mesh(gl, unsafe { mesh.as_ref() }.unwrap(), scene))
        }
    }
    fn process_mesh(&self, gl: &Gl, mesh: &aiMesh, scene: &aiScene) -> Mesh {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut textures: Vec<Texture> = vec![];

        let ai_vertices =
            unsafe { slice::from_raw_parts(mesh.mVertices, mesh.mNumVertices as usize) };

        for (index, vertex) in ai_vertices.iter().enumerate() {
            vertices.push(Vertex {
                normal: unsafe {
                    let ai_normal = *mesh.mNormals.add(index);
                    let aiVector3D { x, y, z } = ai_normal;
                    Vec3 { x, y, z }
                },
                position: {
                    let aiVector3D { x, y, z } = *vertex;
                    Vec3 { x, y, z }
                },
                tex_coords: unsafe {
                    assert!(!mesh.mTextureCoords[0].is_null());
                    let texture_coord = mesh.mTextureCoords[0].add(index);
                    let aiVector3D { x, y, z: _ } = *texture_coord;
                    Vec2 { x, y }
                },
            });
        }
        let ai_faces = unsafe { slice::from_raw_parts(mesh.mFaces, mesh.mNumFaces as usize) };
        for ai_face in ai_faces {
            let ai_indices =
                unsafe { slice::from_raw_parts(ai_face.mIndices, ai_face.mNumIndices as usize) };
            for index in ai_indices {
                indices.push(*index);
            }
        }

        let material = unsafe { *scene.mMaterials.add(mesh.mMaterialIndex as usize) };

        Mesh::new(gl, vertices, indices, textures)
    }
}

fn load_material_textures(
    gl: &Gl,
    mat: *const aiMaterial,
    texture_type: aiTextureType,
    type_name: &str,
) -> Vec<Texture> {
    let textures = vec![];
    let texture_count = unsafe { aiGetMaterialTextureCount(mat, texture_type) };
    for i in 0..texture_count {
        let mut path: aiString = unsafe { zeroed() };
        unsafe {
            aiGetMaterialTexture(
                mat,
                texture_type,
                i,
                &mut path,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        dbg!(path);
        //TextureManager::new().create_texture(gl,)
        //texture = Texture {
        //    id = 



        //};
    }

    textures
}

#[cfg(test)]
mod test {

    fn test_backpack_import() {

    }


}
