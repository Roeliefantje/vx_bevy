#![allow(
    clippy::type_complexity,
    clippy::manual_clamp,
    clippy::module_inception
)]

use std::f32::consts::PI;

use bevy::{core_pipeline::fxaa::Fxaa, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*};
use bevy::render::*;
use bevy::render::settings::*;

mod debug;
mod voxel;

fn main() {
    let mut app = App::default();
    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
        backends:Some(Backends::VULKAN),
                        ..default()
                })
        }))
        //.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(WireframePlugin)
        .add_plugins(voxel::VoxelWorldPlugin)
        .add_plugins(debug::DebugUIPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, configure_wireframe)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera3dBundle {
        projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.,
            far: 2048.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(voxel::player::PlayerController::default())
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());


    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
    
}

fn configure_wireframe(mut wireframe_config: ResMut<WireframeConfig>) {
    // Enable wireframe globally
    wireframe_config.global = true;
}