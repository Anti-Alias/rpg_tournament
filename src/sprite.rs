use bevy::math::{Affine3A, Vec3A};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::primitives::Aabb;
use bevy::render::view::VisibilitySystems;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use crate::screen::Keep;

/// Renders batches of sprites to 3D meshes.
pub fn sprite_plugin(app: &mut App) {
    app.init_resource::<SpriteBatches>();
    app.add_systems(PostUpdate, batch_sprites
        .after(TransformSystem::TransformPropagate)
        .after(VisibilitySystems::CheckVisibility)
    );
}


// Collects sprite entities into the SpriteBatches resource.
fn batch_sprites(
    sprites: Query<(&Sprite3D, &GlobalTransform, &Handle<StandardMaterial>, &ViewVisibility, Option<&Anchor>)>,
    mut batches: ResMut<SpriteBatches>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    batches.clear_meshes(&mut meshes);
    for (sprite, sprite_transf, sprite_mat_handle, visibility, anchor) in &sprites {
        let is_sprite_visible = visibility.get();
        if !is_sprite_visible { continue };
        let Some(sprite_mat) = materials.get(sprite_mat_handle) else { continue };
        let Some(sprite_image_size) = size_of_sprite_mat(sprite_mat, &images) else { continue };
        let sprite_size = sprite.rect.size();
        let sprite_origin: Vec2 = match anchor {
            Some(anchor) => anchor.as_vec() * sprite_size,
            None => Vec2::new(0.5, 0.5) * sprite_size,
        };
        batches.batch_sprite(
            sprite,
            sprite_mat_handle,
            sprite_size,
            sprite_image_size,
            sprite_origin,
            sprite_transf.affine(),
            &mut meshes,
            &mut commands,
        );
    }
}

// Size of a sprite's material.
// Attepts to use the size of the base color texture
fn size_of_sprite_mat(sprite_mat: &StandardMaterial, images: &Assets<Image>) -> Option<Vec2> {
    sprite_mat.base_color_texture
        .as_ref()
        .and_then(|tex_handle| images.get(tex_handle))
        .map(|tex| tex.size_f32())
}


#[derive(Component, Clone, PartialEq, Default, Debug)]
pub struct Sprite3D {
    pub color: Color,   // Color of the sprite
    pub rect: Rect,     // Region of the image to show
}

/// Bundle of components representing a sprite game object in a 3D world.
#[derive(Bundle, Default, Debug)]
pub struct Sprite3DBundle {
    pub sprite: Sprite3D,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}


#[derive(Resource, Default, Debug)]
pub struct SpriteBatches {
    mesh_batches: HashMap<Handle<StandardMaterial>, Handle<Mesh>>,  // Each mesh represents a batch of sprite data (quad) for a single material.
}

impl SpriteBatches {

    pub fn batch_sprite(
        &mut self,
        sprite: &Sprite3D,
        sprite_mat: &Handle<StandardMaterial>,
        sprite_size: Vec2,
        sprite_image_size: Vec2,
        sprite_origin: Vec2,
        sprite_transf: Affine3A,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
    ) {
        // Get or create mesh associated with sprite's material
        let mesh_handle: &Handle<Mesh> = self.mesh_for_material(sprite_mat, meshes, commands);
        let mesh = meshes.get_mut(mesh_handle).unwrap();

        // Generate sprite vertex data
        let sprite_offset = sprite_size * (sprite_origin - 0.5);;
        let sprite_uvs = get_rect_points(flip(sprite.rect)).map(|point| point / sprite_image_size);
        let sprite_positions = get_rect_points(sprite.rect)
            .map(|point2d| point2d.extend(0.0))
            .map(|point3d| point3d + sprite_offset.extend(0.0))
            .map(|point3d| sprite_transf.transform_point3(point3d));
        let sprite_normal = {
            let origin = sprite_positions[0];
            let a = sprite_positions[1] - origin;
            let b = sprite_positions[2] - origin;
            a.cross(b).normalize()
        };

        // Converts vertex data to arrays
        let sprite_positions = sprite_positions.map(|pos| pos.to_array());
        let sprite_normal = sprite_normal.to_array();
        let sprite_color = sprite.color.as_linear_rgba_f32();
        let sprite_uvs = sprite_uvs.map(|uv| uv.to_array());

        // Appends sprite vertex data to mesh
        let indices = get_indices(mesh);
        let i = indices.len() as u16;
        get_indices(mesh).extend([i+0, i+1, i+2, i+2, i+3, i+0]);
        get_position_values(mesh).extend(sprite_positions);
        get_normal_values(mesh).extend([sprite_normal, sprite_normal, sprite_normal, sprite_normal]);
        get_color_values(mesh).extend([sprite_color, sprite_color, sprite_color, sprite_color]);
        get_uv_values(mesh).extend(sprite_uvs);
    }

    // Gets or creates mesh associated with sprite's material.
    // If it does not exist, creates one on the fly and spawns an entity with that mesh.
    fn mesh_for_material(
        &mut self,
        sprite_mat: &Handle<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
    ) -> &Handle<Mesh> {
        self.mesh_batches
            .entry(sprite_mat.clone_weak())
            .or_insert_with(|| {
                let mesh_handle = meshes.add(default_mesh());
                let mesh_pbr = PbrBundle { mesh: mesh_handle.clone(), material: sprite_mat.clone(), ..default() };
                let mesh_aabb = Aabb { center: Vec3A::ZERO, half_extents: Vec3A::new(f32::MAX, f32::MAX, f32::MAX) };
                commands.spawn((mesh_pbr, mesh_aabb, Keep, Name::new("Sprite Batch")));
                mesh_handle
            })
    }

    fn clear_meshes(&mut self, meshes: &mut Assets<Mesh>) {
        for mesh_handle in self.mesh_batches.values() {
            let mesh = meshes.get_mut(mesh_handle).unwrap();
            *mesh = default_mesh();
        }
    }
}

#[inline]
fn get_indices(mesh: &mut Mesh) -> &mut Vec<u16> {
    let indices = mesh.indices_mut().expect("No index data");
    match indices {
        Indices::U16(indices) => indices,
        Indices::U32(_) => panic!("Incorrect index format"),
    }
}

#[inline]
fn get_position_values(mesh: &mut Mesh) -> &mut Vec<[f32; 3]> {
    let attribute_values = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).expect("No position data");
    match attribute_values {
        VertexAttributeValues::Float32x3(values) => values,
        _ => panic!("Incorrect attribute value type"),
    }
}

#[inline]
fn get_normal_values(mesh: &mut Mesh) -> &mut Vec<[f32; 3]> {
    let attribute_values = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL).expect("No normal data");
    match attribute_values {
        VertexAttributeValues::Float32x3(values) => values,
        _ => panic!("Incorrect attribute value type"),
    }
}

#[inline]
fn get_color_values(mesh: &mut Mesh) -> &mut Vec<[f32; 4]> {
    let attribute_values = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR).expect("No color data");
    match attribute_values {
        VertexAttributeValues::Float32x4(values) => values,
        _ => panic!("Incorrect attribute value type"),
    }
}

#[inline]
fn get_uv_values(mesh: &mut Mesh) -> &mut Vec<[f32; 2]> {
    let attribute_values = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).expect("No uv data");
    match attribute_values {
        VertexAttributeValues::Float32x2(values) => values,
        _ => panic!("Incorrect attribute value type"),
    }
}

#[inline]
fn flip(mut rect: Rect) -> Rect {
    std::mem::swap(&mut rect.min.y, &mut rect.max.y);
    rect
}

#[inline]
fn get_rect_points(rect: Rect) -> [Vec2; 4] {
    [
        Vec2::new(rect.min.x, rect.min.y),
        Vec2::new(rect.max.x, rect.min.y),
        Vec2::new(rect.max.x, rect.max.y),
        Vec2::new(rect.min.x, rect.max.y),
    ]
}

// fn get_sprite_normal(rect: Rect)


fn default_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(Indices::U16(vec![]));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<Vec3>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, Vec::<Vec3>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<Vec4>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<Vec2>::new());
    mesh
}
