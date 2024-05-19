use std::f32::consts;
use bevy::ecs::system::CommandQueue;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::sprite::Anchor;

use crate::animation::AnimationBundle;
use crate::asset::CommonAssets;
use crate::ext::EntityCommandsExt;
use crate::ext::WorldExt;
use crate::screen::FadeToScreen;
use crate::screen::ScreenState;
use crate::spawn::AssetBatch;
use crate::ext::CommandsExt;
use crate::screen::ScreenEvent;
use crate::task::Start;
use crate::ui::*;
use crate::dsl::*;


pub fn setup_playground_screen(mut commands: Commands) {
    commands.spawn_task(Start::new(|_, tq| {
        tq.spawn_batch(spawn_playground);
        tq.send_event(ScreenEvent::FinishedLoading);
    }));
}

fn spawn_playground(world: &mut World, commands: &mut CommandQueue, assets: &mut AssetBatch) {

    // Materials
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let plane_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    // Meshes
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let plane_mesh = meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y }));

    // Animations
    let common_assets = world.resource::<CommonAssets>();
    let player_animations = common_assets.player_animations.clone();

    // Bundles
    let mut plane = PbrBundle::default();
    plane.material = plane_mat;
    plane.mesh = plane_mesh;
    plane.transform.scale = Vec3::new(100.0, 1.0, 100.0);
    plane.transform.translation.y = -16.0;

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.directional_light.illuminance /= 2.0;
    dir_light.directional_light.shadows_enabled = true;
    dir_light.transform.rotate_y(-consts::PI / 4.0);
    dir_light.transform.rotate_x(-consts::PI / 4.0);

    let mut player = AnimationBundle::default();
    player.animations = player_animations;
    player.animation_state.animation_index = 0;
    player.material = assets.load("human/material.ron.stdmat");

    // Spawn game objects
    let mut commands = Commands::new(commands, world);
    let aabb = Aabb { center: Vec3A::ZERO, half_extents: Vec3A::splat(32.0) };
    commands.spawn((plane, Name::new("Plane")));
    commands.spawn((dir_light, Name::new("Dir Light")));
    commands.spawn((player, aabb, Anchor::Center, Name::new("Sprite")));

    // Spawn UI
    let t = &mut TreeBuilder::root(&mut commands);
    let back_button: Entity;
    node(c_root, t); insert(Name::new("Title UI"), t); begin(t);
        menu_button("Back", assets, t); back_button=last(t);
    end(t);

    // UI handlers
    commands.entity(back_button).on_press(|world| {
        let task = FadeToScreen(ScreenState::Title);
        world.spawn_task(task);
    });
}


pub fn c_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::End;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::NONE.into();
}