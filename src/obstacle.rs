use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const HAY_BALE_COUNT: i32 = 1;

const COW_COUNT: i32 = 1;

pub fn spawn_hay_bales(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut _rng = rand::thread_rng();

    let bale_gltf: Handle<Scene> = asset_server.load("hay-bale.glb#Scene0");

    for _ in 0..HAY_BALE_COUNT {
        commands
            .spawn()
            .insert_bundle(SpatialBundle::from(
                Transform::from_xyz(0.0, 8.0, 40.0)
                    .with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cylinder(3.72, 4.5))
            .insert(Restitution::coefficient(0.7))
            .insert(ColliderMassProperties::Density(10.0))
            .with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: bale_gltf.clone(),
                    transform: Transform::from_xyz(4.5, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(90.0_f32.to_radians()))
                        .with_scale(Vec3::new(6.0, 6.0, 6.0)),
                    ..Default::default()
                });
            });
    }
}

pub fn spawn_cows(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut _rng = rand::thread_rng();

    let bale_gltf: Handle<Scene> = asset_server.load("cow.glb#Scene0");

    for _ in 0..COW_COUNT {
        commands
            .spawn()
            .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 8.0, -40.0)))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(4.0))
            .insert(Restitution::coefficient(0.7))
            .insert(ColliderMassProperties::Density(5.0))
            .with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: bale_gltf.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                        .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                    ..Default::default()
                });
            });
    }
}
