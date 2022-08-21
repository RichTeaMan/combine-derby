use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera::CombineCamera;

#[derive(Component, Default)]
pub struct Combine {
    pub player_controlled: bool,
    pub engine_force: f32,
    pub reverse_engine_force: f32,
    pub steering_force: f32,
}

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

    /*
        let x = Vector::x_axis();
    let joint = RevoluteJointBuilder::new(x)
        .local_anchor1(Vec3::new(0.0, 0.0, 1.0))
        .local_anchor2(Vec3::new(0.0, 0.0, -3.0));
    commands.spawn()
        .insert(RigidBody::Dynamic)
        .insert(ImpulseJoint::new(parent_entity, joint));
        */

    let body_entity = commands
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
        //.insert(Collider::cuboid(4.6, 4.0, 9.0))
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
            /*
            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(-2.0, -1.0, 2.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });*/

            parent.spawn_bundle(PbrBundle {
                mesh: sphere_handle.clone(),
                material: red_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -20.0),
                ..default()
            });

            parent
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 20.0, 30.0)
                        .with_rotation(Quat::from_rotation_x(-0.4)),
                    ..Default::default()
                })
                .insert(CombineCamera);
        })
        .id();

    let wheel_1_entity = commands
        .spawn_bundle(TransformBundle::from(
            Transform::from_xyz(0.0, 20.0, 0.0), //.with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
        ))
        //   .with_children(|c_parent| {
        //    c_parent
        //        .spawn()
        .insert_bundle(SpatialBundle::from(
            Transform::from_xyz(20.0, 20.0, 0.0), //    .with_rotation(Quat::from_rotation_x(-90.0_f32.to_radians()))
                                                  //    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                                                  //    .with_rotation(Quat::from_rotation_z(90.0_f32.to_radians()))
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(RigidBody::Dynamic)
        .insert(Collider::round_cylinder(0.25, 2.0, 0.25))
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        //    ;})
        .id();
    let x_shift_1 = 6.5;
    //let y_shift_1 = -1.0;
    let y_shift_1 = 10.0;
    let z_shift_1 = 6.0;

    let x_shift_2 = 0.0;
    let y_shift_2 = 0.0;
    let z_shift_2 = 0.0;
    //let shift = 0.0;

    // Setup four joints.
    let x = Vec3::X;
    let z = Vec3::Z;

    let revs = [
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(x_shift_1, y_shift_1, z_shift_1))
            //.local_anchor1(Transform::from_xyz(0.0,0.0,0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, -z_shift_2))
            .motor_velocity(1.0, 1.0),
        //RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-z_shift, 0.0, 0.0)),
        //RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -z_shift)),
        //RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(z_shift, 0.0, 0.0)),
    ];

    //   commands
    //       .entity(wheel_1_entity)
    //       .insert(ImpulseJoint::new(body_entity, revs[0]));
}

pub fn spawn_combine_wheel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let body_gltf: Handle<Scene> = asset_server.load("basic-combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 16,
        stacks: 16,
    }));
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });

    /*
        let x = Vector::x_axis();
    let joint = RevoluteJointBuilder::new(x)
        .local_anchor1(Vec3::new(0.0, 0.0, 1.0))
        .local_anchor2(Vec3::new(0.0, 0.0, -3.0));
    commands.spawn()
        .insert(RigidBody::Dynamic)
        .insert(ImpulseJoint::new(parent_entity, joint));
        */

    let body_entity = commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 20.0, 0.0)))
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
            /*parent.spawn_bundle(SceneBundle {
                scene: body_gltf,
                transform: Transform::from_xyz(0.0, -1.7, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });*/

            parent
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 20.0, 30.0)
                        .with_rotation(Quat::from_rotation_x(-0.4)),
                    ..Default::default()
                })
                .insert(CombineCamera);
        })
        .id();

    /*
        let wheel_1_entity = commands
        .spawn()
        .insert(Transform::from_xyz(100000.0, 30.0, 30.0))
        .insert		(GlobalTransform::default())
        .with_children(|parent|{
            parent.spawn()
         //   .insert(
         //       Transform::from_xyz(0.0, 20.0, 0.0)
                    //.with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
         //   )
                    .insert(Restitution::coefficient(0.7))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::round_cylinder(0.25, 2.0, 0.25))
                    .insert(GravityScale(0.0))
                    .insert(ColliderMassProperties::Density(2.0))
                    .with_children(|parent| {

                    });
                })
           .id();
    */
    /*
           let wheel_2_entity = commands.spawn_bundle(PbrBundle {
            mesh: sphere_handle.clone(),
            material: red_material_handle.clone(),
            transform: Transform::from_xyz(20.0, 20.0, 0.0),
            ..default()
        }).with_children(|parent|{
            parent.spawn()
            .insert(
                Transform::from_xyz(0.0, 0.0, 0.0)
                    //.with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
            )
                    .insert(Restitution::coefficient(0.7))
                    .insert(RigidBody::Fixed)
                    .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                    .insert(GravityScale(0.0))
                    .insert(ColliderMassProperties::Density(2.0))
                    .with_children(|parent| {

                    });
                });
    */

    let wheel_restitution = 0.1;

    let wheel_0_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution));

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_1_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution));

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_2_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution));

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_3_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution));

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let x_shift_1 = 6.5;
    let y_shift_1 = 4.0;
    //let y_shift_1 = -20.0;
    let z_shift_1 = 6.0;

    let x_shift_2 = 0.0;
    let y_shift_2 = 0.0;
    let z_shift_2 = 0.0;
    //let shift = 0.0;

    // Setup four joints.
    let x = Vec3::X;
    let z = Vec3::Z;

    let velocity = 15.0;
    let max_force = f32::MAX; // 10000000.0;

    let factor = 0.0;

    let revs = [
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, -z_shift_1))
            //.local_anchor1(Transform::from_xyz(0.0,0.0,0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, -z_shift_1))
            //.local_anchor1(Transform::from_xyz(0.0,0.0,0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, z_shift_1))
            //.local_anchor1(Transform::from_xyz(0.0,0.0,0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, z_shift_1))
            //.local_anchor1(Transform::from_xyz(0.0,0.0,0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
    ];

    commands
        .entity(wheel_0_entity)
        .insert(ImpulseJoint::new(body_entity, revs[0]));

    commands
        .entity(wheel_1_entity)
        .insert(ImpulseJoint::new(body_entity, revs[1]));

    commands
        .entity(wheel_2_entity)
        .insert(ImpulseJoint::new(body_entity, revs[2]));

    commands
        .entity(wheel_3_entity)
        .insert(ImpulseJoint::new(body_entity, revs[3]));
}
