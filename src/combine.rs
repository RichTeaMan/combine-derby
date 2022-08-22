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

#[derive(Component)]
pub struct SteeringWheel {
    pub steering_wheel_position: SteeringWheelPosition,
}

#[derive(Component)]
pub struct DrivingWheel;

#[derive(Eq, PartialEq)]
pub enum SteeringWheelPosition {
    Left,
    Right,
}


pub fn spawn_combine(mut commands: Commands, asset_server: Res<AssetServer>) {
    let physics = RigidBody::Dynamic;

    let body_gltf: Handle<Scene> = asset_server.load("basic-combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let body_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 20.0, 0.0)))
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
        .insert(physics)
        .insert(Collider::cuboid(4.6, 4.0, 9.0))
        .insert(ColliderMassProperties::Density(20.0))
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 4.0,
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: body_gltf,
                transform: Transform::from_xyz(0.0, -3.1, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });

            parent
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 20.0, 40.0)
                        .with_rotation(Quat::from_rotation_x(-0.4)),
                    ..Default::default()
                })
                .insert(CombineCamera);
        })
        .id();

    let wheel_restitution = 0.3;

    let wheel_friction = 0.8;

    let wheel_0_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction));

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
        .insert(physics)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction));

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
        .insert(physics)
        .insert(ColliderMassProperties::Density(0.5))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(ColliderMassProperties::Density(0.5))
                .insert(Friction::coefficient(wheel_friction));

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
        .insert(physics)
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction));

            parent.spawn_bundle(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let steering_rack_left = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(physics)
        .id();

    let steering_rack_right = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(physics)
        .id();

    let x_shift_1 = 6.5;
    let y_shift_1 = 4.0;
    //let y_shift_1 = -20.0;
    let z_shift_1 = 8.0;
    let z_shift_rear = 5.0;

    let x_shift_2 = 0.0;
    let y_shift_2 = 0.0;
    let z_shift_2 = 0.0;
    let x = Vec3::X;

    let velocity = 0.0;
    let max_force = 50000.0; // f32::MAX / 2.0;
    let max_steer_force = f32::MAX;

    let factor = 0.0;

    let revs = [
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, -z_shift_1))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, -z_shift_1))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(1.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-1.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, factor)
            .motor_max_force(max_force),
    ];

    let steering_left_joint = RevoluteJointBuilder::new(Vec3::Y)
        .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, z_shift_rear))
        .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
        .motor_position(0.0, 0.5, 0.5)
        .motor_max_force(max_steer_force);

    let steering_right_joint = RevoluteJointBuilder::new(Vec3::Y)
        .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, z_shift_rear))
        .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
        .motor_position(0.0, 0.5, 0.5)
        .motor_max_force(max_steer_force);

    commands
        .entity(steering_rack_left)
        .insert(physics)
        .insert(MultibodyJoint::new(body_entity, steering_left_joint))
        .insert(SteeringWheel {
            steering_wheel_position: SteeringWheelPosition::Left,
        });

    commands
        .entity(steering_rack_right)
        .insert(physics)
        .insert(MultibodyJoint::new(body_entity, steering_right_joint))
        .insert(SteeringWheel {
            steering_wheel_position: SteeringWheelPosition::Right,
        });

    commands
        .entity(wheel_0_entity)
        .insert(MultibodyJoint::new(body_entity, revs[0]))
        .insert(DrivingWheel);

    commands
        .entity(wheel_1_entity)
        .insert(MultibodyJoint::new(body_entity, revs[1]))
        .insert(DrivingWheel);

    commands
        .entity(wheel_2_entity)
        .insert(MultibodyJoint::new(steering_rack_left, revs[2]))
        .insert(DrivingWheel);

    commands
        .entity(wheel_3_entity)
        .insert(MultibodyJoint::new(steering_rack_right, revs[3]))
        .insert(DrivingWheel);
}
