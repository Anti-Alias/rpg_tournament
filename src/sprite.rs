use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;

/// Renders batches of sprites to 3D meshes.
pub fn sprite_plugin(app: &mut App) {
    app.init_resource::<SpriteBatches>();
    app.add_systems(PostUpdate, batch_sprites);
}


// Collects sprite entities into the SpriteBatches resource.
fn batch_sprites(
    sprites: Query<(&Sprite3D, &GlobalTransform, &Handle<StandardMaterial>, Option<&Anchor>)>,
    mut batches: ResMut<SpriteBatches>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    images: Res<Assets<Image>>,
) {
    batches.clear_meshes(&mut meshes);
    for (sprite, sprite_transf, sprite_mat_handle, anchor) in &sprites {
        let Some(sprite_mat) = materials.get(sprite_mat_handle) else { continue };
        let sprite_size: Vec2 = {
            let Some(sprite_base_col_handle) = &sprite_mat.base_color_texture else { continue };
            let Some(sprite_base_col_img) = images.get(sprite_base_col_handle) else { continue };
            sprite_base_col_img.size_f32()
        };
        let sprite_origin: Vec2 = match anchor {
            Some(anchor) => anchor.as_vec() * sprite_size,
            None => Vec2::new(0.5, 0.5) * sprite_size,
        };
        batches.batch_sprite(
            sprite,
            sprite_mat,
            sprite_size,
            sprite_origin,
            sprite_transf,
            &mut meshes,
            &mut commands,
        );
    }
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
    meshes: HashMap<UntypedHandle, Handle<Mesh>>,
}

impl SpriteBatches {

    pub fn batch_sprite(
        &mut self,
        sprite: &Sprite3D,
        sprite_mat: &StandardMaterial,
        sprite_size: Vec2,
        sprite_origin: Vec2,
        sprite_transf: &GlobalTransform,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
    ) {
        println!("Sprite: {sprite:?}");
    }

    pub fn clear_meshes(&mut self, meshes: &mut Assets<Mesh>) {
        for mesh_handle in self.meshes.values() {
            let mesh = meshes.get_mut(mesh_handle).unwrap();
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
        }
    }
}

/// Marks a mesh entity as a batch of sprites with the same material.
/// to be updated every frame.
pub struct SpriteBatch;