use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct CommonAssets {
    pub meshes: CommonMeshes,
    pub materials: CommonMaterials,
}

impl FromWorld for CommonAssets {
    fn from_world(world: &mut World) -> Self {

        let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
        let meshes = CommonMeshes {
            sphere: mesh_assets.add(Sphere::new(1.0)),
        };

        let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
        let materials = CommonMaterials {
            white: material_assets.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                ..default()
            }),
        };
        Self { meshes, materials }
    }
}

#[derive(Debug)]
pub struct CommonMeshes {
    pub sphere: Handle<Mesh>,
}

#[derive(Debug)]
pub struct CommonMaterials {
    pub white: Handle<StandardMaterial>,
}