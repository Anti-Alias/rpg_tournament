use std::hash::Hash;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;

use super::{RegularTile, Strip, TH};

//pub type CollisionMesh = HashMesh<CollisionVertex>;
pub type GraphicsMesh = HashMesh<GraphicsVertex>;


/// A mesh builder used for building collision and graphics meshes.
/// De-duplicates vertices upon insertion using a [`HashMap`].
pub struct HashMesh<V> {
    vertices: Vec<V>,
    indices: Vec<u32>,
    indices_of_vertices: HashMap<V, u32>,
}

impl<V> HashMesh<V> {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            indices_of_vertices: HashMap::new(),
        }
    }

    pub fn finish(self) -> (Vec<V>, Vec<u32>) {
        (
            self.vertices,
            self.indices
        )
    }
}

impl<V: Vertex> HashMesh<V> {

    /// Adds 6 vertices as a quad (two triangles).
    pub fn push_quad(&mut self, a: V, b: V, c: V, d: V, e: V, f: V) {
        self.push_tri(a, b, c);
        self.push_tri(d, e, f);
    }
    
    fn push_tri(&mut self, a: V, b: V, c: V) {
        if V::is_tri_empty(a, b, c) { return };
        let ai = self.push_vertex(a);
        let bi = self.push_vertex(b);
        let ci = self.push_vertex(c);
        self.indices.extend([ai, bi, ci]);
    }

    fn push_vertex(&mut self, v: V) -> u32 {
        match self.indices_of_vertices.get(&v) {
            Some(idx_of_vert) => *idx_of_vert,
            None => {
                let idx_of_vert = self.vertices.len() as u32;
                self.vertices.push(v);
                self.indices_of_vertices.insert(v, idx_of_vert);
                idx_of_vert
            },
        }
    }
}


pub fn create_bevy_mesh(
    gmesh: GraphicsMesh,
    material_width: f32,
    material_height: f32,
    tile_width: f32,
    tile_height: f32
) -> Mesh {
    let (gmesh_verts, gmesh_indices) = gmesh.finish();
    let scale_yz = tile_height as f32 / TH as f32;
    let i_mat_width = 1.0 / material_width;
    let i_mat_height = 1.0 / material_height;
    let positions: Vec<[f32; 3]> = gmesh_verts.iter()
        .map(|gvert| [
            gvert.pos.x as f32 * tile_width,
            gvert.pos.y as f32 * scale_yz,
            -gvert.pos.z as f32 * scale_yz,
        ])
        .collect();
    let normals: Vec<[f32; 3]> = gmesh_verts.iter()
        .map(|gvert| {
            let norm = Vec3::new(
                gvert.pos.x as f32 * (1.0 / tile_width),
                gvert.pos.y as f32 * (1.0 / tile_height),
                gvert.pos.z as f32 * (1.0 / tile_height),
            ).normalize();
            [ norm.x, norm.y, norm.z ]
        })
        .collect();
    let uvs: Vec<[f32; 2]> = gmesh_verts.iter()
        .map(|gvert| [
            gvert.uv.x as f32 * i_mat_width,
            gvert.uv.y as f32 / i_mat_height,
        ])
        .collect();
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(gmesh_indices));
    mesh
}

pub trait Vertex: Copy + Eq + Hash {
    /// True if the area of the triangle formed by three vertices is zero.
    /// Used for culling unnecessary geometry.
    fn is_tri_empty(a: Self, b: Self, c: Self) -> bool;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct CollisionVertex(IVec3);
impl Vertex for CollisionVertex {
    fn is_tri_empty(a: Self, b: Self, c: Self) -> bool {
        let d = b.0 - a.0;
        let e = c.0 - a.0;
        let cross = d.cross(e);
        cross.x == 0 && cross.y == 0 && cross.z == 0
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct GraphicsVertex {
    pub pos: IVec3,
    pub norm: IVec3,
    pub uv: IVec2,
}

impl GraphicsVertex {

    //   E -------- D,C
    //   |          /|
    //   |       /   |
    //   |    /      |
    //   | /         |
    //  F,A -------- B
    pub fn quad(positions: [IVec3; 4], uvs: [IVec2; 4]) -> (Self, Self, Self, Self, Self, Self) {
        let pos_a = positions[0];
        let pos_b = positions[1];
        let pos_c = positions[2];
        let pos_d = positions[2];
        let pos_e = positions[3];
        let pos_f = positions[0];
        let norm_abc = (pos_b - pos_a).cross(pos_c - pos_a);
        let norm_def = (pos_e - pos_d).cross(pos_f - pos_d);
        let a = Self { pos: pos_a, norm: norm_abc, uv: uvs[0] };
        let b = Self { pos: pos_b, norm: norm_abc, uv: uvs[1] };
        let c = Self { pos: pos_c, norm: norm_abc, uv: uvs[2] };
        let d = Self { pos: pos_d, norm: norm_def, uv: uvs[2] };
        let e = Self { pos: pos_e, norm: norm_def, uv: uvs[3] };
        let f = Self { pos: pos_f, norm: norm_def, uv: uvs[0] };
        (a, b, c, d, e, f)
    }
}

impl Vertex for GraphicsVertex {
    fn is_tri_empty(a: Self, b: Self, c: Self) -> bool {
        let d = b.pos - a.pos;
        let e = c.pos - a.pos;
        let cross = d.cross(e);
        cross.x == 0 && cross.y == 0 && cross.z == 0
    }
}