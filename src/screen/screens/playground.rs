use std::f32::consts::PI;

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
    
    let common_assets = world.resource::<CommonAssets>();
    let mut commands = Commands::new(commands, world);

    let mut plane = PbrBundle::default();
    plane.material = assets.add(StandardMaterial::from(Color::GRAY));
    plane.mesh = common_assets.cuboid_mesh.clone();
    plane.transform.scale = Vec3::new(300.0, 32.0, 200.0);
    plane.transform.translation.y = -16.0;
    commands.spawn((plane, Name::new("Floor")));

    let mut red_wall = PbrBundle::default();
    red_wall.material = assets.add(StandardMaterial::from(Color::RED));
    red_wall.mesh = common_assets.cuboid_mesh.clone();
    red_wall.transform.scale = Vec3::new(32.0, 16.0, 32.0);
    red_wall.transform.translation = Vec3::new(-64.0, 8.0, 10.0);
    commands.spawn((red_wall, Name::new("Red Wall")));

    let mut green_wall = PbrBundle::default();
    green_wall.material = assets.add(StandardMaterial::from(Color::GREEN));
    green_wall.mesh = common_assets.cuboid_mesh.clone();
    green_wall.transform.scale = Vec3::new(32.0, 16.0, 32.0);
    green_wall.transform.translation = Vec3::new(0.0, 8.0, -64.0);
    commands.spawn((green_wall, Name::new("Green Wall")));

    let mut blue_wall = PbrBundle::default();
    blue_wall.material = assets.add(StandardMaterial::from(Color::BLUE));
    blue_wall.mesh = common_assets.cuboid_mesh.clone();
    blue_wall.transform.scale = Vec3::new(32.0, 16.0, 32.0);
    blue_wall.transform.translation = Vec3::new(64.0, 8.0, 10.0);
    commands.spawn((blue_wall, Name::new("Blue Wall")));

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.cascade_shadow_config.bounds = vec![256.0];
    dir_light.directional_light.illuminance /= 2.5;
    dir_light.directional_light.shadows_enabled = true;
    dir_light.transform.rotate_y(-0.4);
    dir_light.transform.rotate_x(-1.1);
    commands.spawn((dir_light, Name::new("Dir Light")));

    let player_aabb = Aabb { center: Vec3A::ZERO, half_extents: Vec3A::splat(32.0) };
    let mut player = AnimationBundle::default();
    player.transform.translation.y = 13.0;
    player.animations = common_assets.player_animations.clone();
    player.animation_state.animation_index = 0;
    player.material = assets.load("human/material.ron.stdmat");
    commands.spawn((player, player_aabb, Anchor::Center, Name::new("Sprite")));

    let t = &mut TreeBuilder::root(&mut commands);
    let back_button: Entity;
    node(c_root, t); begin(t);
        menu_button("Back", assets, t); back_button=last(t);
    end(t);

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