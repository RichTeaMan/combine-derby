use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Combine {
    pub player_controlled: bool,
    pub engine_force: f32,
    pub reverse_engine_force: f32,
    pub steering_force: f32,
}

#[derive(Component)]
pub struct CameraTarget;

pub fn spawn_combine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let body_gltf: Handle<Scene> = asset_server.load("basic-combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.0,
        sectors: 2,
        stacks: 2,
    }));
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });

    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 0.0, 0.0),
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Friction::coefficient(0.7))
        .insert(Combine {
            engine_force: 3200000.0,
            reverse_engine_force: 280000.0,
            player_controlled: true,
            steering_force: 12000000.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.6, 4.0, 9.0))
        .insert(ColliderMassProperties::Density(26.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 7.0,
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: body_gltf,
                transform: Transform::from_xyz(0.0, -1.7, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(-2.0, -1.0, 2.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });

            parent
                .spawn_bundle(PbrBundle {
                    mesh: sphere_handle.clone(),
                    material: red_material_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -20.0),
                    ..default()
                })
                .insert(CameraTarget);

            parent.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 20.0, 30.0).with_rotation(Quat::from_rotation_x(-0.4)),
                ..Default::default()
            });
        });
}
