use std::hash::Hash;
use bevy::math::{I16Vec2, I16Vec3};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;

use super::TH;

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
    pub fn push_quad(&mut self, quad: [V; 6]) {
        self.push_tri([quad[0], quad[1], quad[2]]);
        self.push_tri([quad[3], quad[4], quad[5]]);
    }
    
    fn push_tri(&mut self, tri: [V; 3]) {
        if V::is_tri_empty(tri[0], tri[1], tri[2]) { return };
        let i0 = self.push_vertex(tri[0]);
        let i1 = self.push_vertex(tri[1]);
        let i2 = self.push_vertex(tri[2]);
        self.indices.extend([i0, i1, i2]);
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
    const EPS: f32 = 0.001;
    let (gmesh_verts, gmesh_indices) = gmesh.finish();
    let scale_yz = tile_height as f32 / TH as f32;
    let positions: Vec<[f32; 3]> = gmesh_verts.iter()
        .map(|gvert| [
            gvert.pos.x as f32 * tile_width,
            gvert.pos.y as f32 * scale_yz + gvert.offset as f32 * EPS,
            gvert.pos.z as f32 * scale_yz + gvert.offset as f32 * EPS,
        ])
        .collect();
    let normals: Vec<[f32; 3]> = gmesh_verts.iter()
        .map(|gvert| {
            let norm = gvert.norm.as_vec3() * Vec3::new(0.0, 1.0 / scale_yz, 1.0 / scale_yz);
            let norm = norm.normalize();
            [ norm.x, norm.y, norm.z ]
        })
        .collect();
    let uvs: Vec<[f32; 2]> = gmesh_verts.iter()
        .map(|gvert| [
            gvert.uv.x as f32 * (1.0 / material_width),
            gvert.uv.y as f32 * (1.0 / material_height),
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
    pub pos: I16Vec3,
    pub norm: I16Vec3,
    pub uv: I16Vec2,
    pub offset: u16
}

impl GraphicsVertex {

    //   E -------- D,C
    //   |          /|
    //   |       /   |
    //   |    /      |
    //   | /         |
    //  F,A -------- B
    pub fn quad(positions: [I16Vec3; 4], uvs: [I16Vec2; 4], offset: u16) -> [Self; 6] {
        let pos_a = positions[0];
        let pos_b = positions[1];
        let pos_c = positions[2];
        let pos_d = positions[2];
        let pos_e = positions[3];
        let pos_f = positions[0];
        let norm_abc = (pos_b - pos_a).cross(pos_c - pos_a);
        let norm_def = (pos_d - pos_f).cross(pos_e - pos_f);
        let a = Self { pos: pos_a, norm: norm_abc, uv: uvs[0], offset };
        let b = Self { pos: pos_b, norm: norm_abc, uv: uvs[1], offset };
        let c = Self { pos: pos_c, norm: norm_abc, uv: uvs[2], offset };
        let d = Self { pos: pos_d, norm: norm_def, uv: uvs[2], offset };
        let e = Self { pos: pos_e, norm: norm_def, uv: uvs[3], offset };
        let f = Self { pos: pos_f, norm: norm_def, uv: uvs[0], offset };
        [a, b, c, d, e, f]
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