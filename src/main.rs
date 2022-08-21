mod camera;
mod combine;
mod control;
mod events;
mod input;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::{SwitchCameraEvent, camera_events};
use combine::{spawn_combine, spawn_combine_wheel};
use control::{ control_events};
use events::ControlEvent;
use input::keyboard_input;

const PLANE_SIZE: f32 = 1000.0;

#[derive(Component)]
pub struct DebugPointer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_event::<ControlEvent>()
        .add_event::<SwitchCameraEvent>()
        //.add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_combine_wheel)
        .add_startup_system(camera::spawn_camera)
        .add_system(camera::pan_orbit_camera)
        //.add_system(update_camera)
        //.add_system(move_camera)
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_input)
        .add_system(control_events)
        .add_system(camera_events)
        .run();
}
fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const TILE_SIZE: f32 = 20.0;
    let ground_plane_handle = meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE }));

    let ground_texture = asset_server.load("ground_texture.png");

    let ground_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.31, 0.21, 0.14),
        base_color_texture: Some(ground_texture),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });

    let ground_y_position = -2.0;

    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(PLANE_SIZE, 0.1, PLANE_SIZE))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            ground_y_position,
            0.0,
        )));

    let tile_count = (PLANE_SIZE / TILE_SIZE) as i32;
    for i in -tile_count..tile_count {
        for j in -tile_count..tile_count {
            commands.spawn_bundle(PbrBundle {
                mesh: ground_plane_handle.clone(),
                // Change material according to position to get alternating pattern
                material: ground_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 * TILE_SIZE) as f32,
                    ground_y_position,
                    (j as f32 * TILE_SIZE) as f32,
                )),
                ..default()
            });
        }
    }

    /* Create the bouncing balls. */
    for _ in 0..10 {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(1.5))
            .insert(Restitution::coefficient(0.7))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(60.0, 24.0, 0.0)));
    }

    // directional 'sun' light
    const HALF_SIZE: f32 = PLANE_SIZE;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.0,
        sectors: 2,
        stacks: 2,
    }));
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });
}
