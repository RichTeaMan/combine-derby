use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera::CombineCamera;

#[derive(Component, Default)]
pub struct Combine {
    pub combine_id: i32,
}

#[derive(Component)]
pub struct SteeringWheel {
    pub steering_wheel_position: SteeringWheelPosition,
    pub combine_id: i32,
    pub combine_wheel_base: f32,
    pub combine_track_width: f32,
}

#[derive(Component)]
pub struct DrivingWheel {
    pub combine_id: i32,
}

#[derive(Eq, PartialEq)]
pub enum SteeringWheelPosition {
    Left,
    Right,
}

pub fn spawn_combines(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands = create_combine(
        commands,
        &asset_server,
        7,
        Transform::from_xyz(0.0, 10.0, 0.0),
        true,
    );

    create_combine(
        commands,
        &asset_server,
        1,
        Transform::from_xyz(80.0, 10.0, 0.0),
        false,
    );
}

fn create_combine<'w, 's>(
    mut commands: Commands<'w, 's>,
    asset_server: &Res<AssetServer>,
    combine_id: i32,
    spawn_transform: Transform,
    active_camera: bool,
) -> Commands<'w, 's> {
    let body_linear_damping = 0.0;
    let body_angular_damping = 0.0;
    let body_restitution = 0.7;
    let body_friction = 0.7;
    let body_density = 20.0;

    let wheel_restitution = 0.3;
    let wheel_friction = 1.0;
    let wheel_density = 9.0;

    let max_wheel_force = 50000.0;
    let max_steer_force = f32::MAX;
    let wheel_factor = 0.0;

    let steering_motor_stiffness = 0.5;
    let steering_motor_damping = 0.5;

    let physics = RigidBody::Dynamic;

    let body_gltf: Handle<Scene> = asset_server.load("basic-combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let body_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(spawn_transform))
        .insert(Restitution::coefficient(body_restitution))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Friction::coefficient(body_friction))
        .insert(Combine { combine_id })
        .insert(physics)
        .insert(Collider::cuboid(4.6, 4.0, 9.0))
        .insert(ColliderMassProperties::Density(body_density))
        .insert(Damping {
            linear_damping: body_linear_damping,
            angular_damping: body_angular_damping,
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
                    camera: Camera {
                        is_active: active_camera,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CombineCamera { combine_id });
        })
        .id();

    let wheel_0_entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

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
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

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
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

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
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::round_cylinder(0.5, 2.0, 0.25))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

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

    let revs = [
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, -z_shift_1))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, wheel_factor)
            .motor_max_force(max_wheel_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, -z_shift_1))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, wheel_factor)
            .motor_max_force(max_wheel_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(1.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, wheel_factor)
            .motor_max_force(max_wheel_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-1.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
            .motor_velocity(velocity, wheel_factor)
            .motor_max_force(max_wheel_force),
    ];

    let steering_left_joint = RevoluteJointBuilder::new(Vec3::Y)
        .local_anchor1(Vec3::new(x_shift_1, -y_shift_1, z_shift_rear))
        .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
        .motor_position(0.0, steering_motor_stiffness, steering_motor_damping)
        .motor_max_force(max_steer_force);

    let steering_right_joint = RevoluteJointBuilder::new(Vec3::Y)
        .local_anchor1(Vec3::new(-x_shift_1, -y_shift_1, z_shift_rear))
        .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
        .motor_position(0.0, steering_motor_stiffness, steering_motor_damping)
        .motor_max_force(max_steer_force);

    let wheel_base = z_shift_1 + z_shift_rear;
    let track_width = x_shift_1 * 2.0;

    commands
        .entity(steering_rack_left)
        .insert(physics)
        .insert(MultibodyJoint::new(body_entity, steering_left_joint))
        .insert(SteeringWheel {
            steering_wheel_position: SteeringWheelPosition::Left,
            combine_id,
            combine_wheel_base: wheel_base,
            combine_track_width: track_width,
        });

    commands
        .entity(steering_rack_right)
        .insert(physics)
        .insert(MultibodyJoint::new(body_entity, steering_right_joint))
        .insert(SteeringWheel {
            steering_wheel_position: SteeringWheelPosition::Right,
            combine_id,
            combine_wheel_base: wheel_base,
            combine_track_width: track_width,
        });

    commands
        .entity(wheel_0_entity)
        .insert(MultibodyJoint::new(body_entity, revs[0]))
        .insert(DrivingWheel { combine_id });

    commands
        .entity(wheel_1_entity)
        .insert(MultibodyJoint::new(body_entity, revs[1]))
        .insert(DrivingWheel { combine_id });

    commands
        .entity(wheel_2_entity)
        .insert(MultibodyJoint::new(steering_rack_left, revs[2]))
        .insert(DrivingWheel { combine_id });

    commands
        .entity(wheel_3_entity)
        .insert(MultibodyJoint::new(steering_rack_right, revs[3]))
        .insert(DrivingWheel { combine_id });

    commands
}
