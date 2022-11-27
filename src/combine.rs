use std::{
    collections::{HashMap, VecDeque},
    f32::consts::PI,
    time::Duration,
};

use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::Vector};

use crate::{
    ai::AiState,
    arena::{PLANE_SIZE, RAMP_HEIGHT},
    camera::CombineCamera,
};

pub const PLAYER_COMBINE_ID: i32 = 1;

const VELOCITY_QUEUE_LIMIT: usize = 16;

#[derive(Component, Default)]
pub struct Combine {
    pub combine_id: i32,

    pub velocity_queue: VecDeque<(Vec3, Duration)>,

    pub velocity: f32,
}

impl Combine {
    pub fn new(combine_id: i32) -> Combine {
        Combine {
            combine_id,
            velocity_queue: VecDeque::new(),
            velocity: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct SteeringWheel {
    pub steering_wheel_position: SteeringWheelPosition,
    pub combine_id: i32,
    pub combine_wheel_base: f32,
    pub combine_track_width: f32,
}

#[derive(Component)]
pub struct BallVehicle {
    pub combine_id: i32,
    pub drive_force: f32,
    pub forward_vector: Vec3,
    pub steering_angle: f32,
    pub max_steering_angle: f32,
}

impl BallVehicle {
    pub fn facing_vector(&self, origin_vector: Vec3) -> Vec3 {
        
        let a = self.forward_vector.normalize_or_zero();
        let b = origin_vector.normalize_or_zero();
        let g = a - b;
        let fv = g * 20.0;
        info!("facing: {a} - {b} = {s} -> {ss}", s = g, ss = fv);
        fv
    }
}

#[derive(Component)]
pub struct BallVehicleAvatar {
    pub combine_id: i32,
}

#[derive(Component)]
pub struct DrivingWheel {
    pub combine_id: i32,
    pub target_velocity: f32,
}

impl DrivingWheel {
    pub fn new(combine_id: i32) -> DrivingWheel {
        DrivingWheel {
            combine_id,
            target_velocity: 10.0,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum SteeringWheelPosition {
    Left,
    Right,
}

pub fn spawn_combines(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_position_1 = Transform::from_translation(Vec3::new(
        PLANE_SIZE - (PLANE_SIZE / 3.0),
        RAMP_HEIGHT + 4.1,
        PLANE_SIZE - (PLANE_SIZE / 3.0),
    ))
    .with_rotation(Quat::from_rotation_y(45.0_f32.to_radians()));

    commands = create_ball(
        commands,
        &asset_server,
        PLAYER_COMBINE_ID,
        spawn_position_1,
        true,
    );

    let spawn_position_2 = Transform::from_translation(Vec3::new(
        -(PLANE_SIZE - (PLANE_SIZE / 3.0)),
        RAMP_HEIGHT + 4.1,
        -(PLANE_SIZE - (PLANE_SIZE / 3.0)),
    ))
    .with_rotation(Quat::from_rotation_y(215.0_f32.to_radians()));

    create_combine(commands, &asset_server, 2, spawn_position_2, false);
}

#[derive(Resource)]
pub struct DebugResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct DebugCube {
    pub translation: Vec3,
    pub id: String,
}

pub fn setup_shape_debug_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let debug_material_handle = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 1.0,
            blue: 0.0,
            green: 0.0,
            alpha: 1.0,
        },
        ..default()
    });

    commands.insert_resource(DebugResource {
        mesh: debug_cube_handle,
        material: debug_material_handle,
    });
}

pub fn shape_debug_system(
    mut commands: Commands,
    mut writer: EventReader<DebugEvent>,
    debug_resource: Res<DebugResource>,
    mut debug_cubes: Query<(&DebugCube, &mut Transform)>,
) {
    let mut event_map = HashMap::new();
    for debug_event in writer.iter() {
        event_map.insert(&debug_event.id, debug_event.translation);
    }

    for (debug_cube, mut transform) in debug_cubes.iter_mut() {
        if let Some(debug_cube_translation) = event_map.remove(&debug_cube.id) {
            transform.translation = debug_cube_translation.clone();
        }
    }

    for (id, translation) in event_map {
        commands
            .spawn(PbrBundle {
                mesh: debug_resource.mesh.clone(),
                material: debug_resource.material.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            })
            .insert(DebugCube {
                id: id.to_string(),
                translation: translation,
            });
    }
}

fn create_ball<'w, 's>(
    mut commands: Commands<'w, 's>,
    asset_server: &Res<AssetServer>,
    combine_id: i32,
    spawn_transform: Transform,
    active_camera: bool,
) -> Commands<'w, 's> {
    let collider_ball_radius = 4.0;

    let body_linear_damping = 0.0;
    let body_angular_damping = 0.0;
    let body_restitution = 0.7;
    let body_friction = 0.7;
    let body_density = 10.0;

    let wheel_restitution = 0.0;
    let wheel_friction = 1.8;
    let wheel_density = 8.0;

    let wheel_width = 0.2;

    let max_wheel_force = f32::MAX;
    let max_steer_force = f32::MAX;
    let wheel_factor = 0.7;

    let steering_motor_stiffness = 0.5;
    let steering_motor_damping = 0.5;

    let physics = RigidBody::Dynamic;

    let center_of_mass = Vec3::new(0.0, -3.0, -7.0);
    let ballast_mass = 500.0;

    let body_gltf: Handle<Scene> = asset_server.load("combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let mut body_commands = commands.spawn(SpatialBundle::from(spawn_transform));

    body_commands
        .insert(Restitution::coefficient(body_restitution))
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO,
        })
        .insert(BallVehicle {
            combine_id,
            drive_force: 500.0,
            forward_vector: spawn_transform.forward(),
            steering_angle: 0.0,
            max_steering_angle: 1.0,
        })
        .insert(Friction::coefficient(body_friction))
        .insert(Combine::new(combine_id))
        .insert(physics)
        .insert(Collider::ball(collider_ball_radius))
        .insert(ColliderMassProperties::Density(body_density))
        //.insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: body_linear_damping,
            angular_damping: body_angular_damping,
        })
        .with_children(|parent| {
            parent
                .spawn(SpatialBundle::default())
                .insert(BallVehicleAvatar { combine_id })
                .with_children(|parent| {
                    /*
                    parent.spawn(SceneBundle {
                        scene: body_gltf.clone(),
                        transform: Transform::from_xyz(0.0, -3.1, 0.0)
                            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                            .with_scale(Vec3::new(0.1, 0.1, 0.1)),
                        ..Default::default()
                    });
                    */
                    // parent
                    //     .spawn(Camera3dBundle {
                    //         transform: Transform::from_xyz(0.0, 20.0, 40.0)
                    //             .with_rotation(Quat::from_rotation_x(-0.4)),
                    //         camera: Camera {
                    //             is_active: active_camera,
                    //             ..Default::default()
                    //         },
                    //         ..Default::default()
                    //     })
                    //     .insert(CombineCamera { combine_id });
                });
        });

    if combine_id != PLAYER_COMBINE_ID {
        body_commands.insert(AiState {
            combine_id,
            ..default()
        });
    }

    let x = Vec3::X;
    let joint = RevoluteJointBuilder::new(x)
        .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
        .local_anchor2(Vec3::new(0.0, 0.0, 0.0));

    let body_entity = body_commands.id();
    let avatar = commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )))
        .insert(BallVehicleAvatar { combine_id })
        //.insert(RigidBody::Dynamic)
        //.insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_Z)
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: body_gltf,
                transform: Transform::IDENTITY
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 0.2, 0.2)),
                ..Default::default()
            });
            parent
                .spawn(Camera3dBundle {
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

    //commands
    //    .entity(avatar)
    //    .insert(MultibodyJoint::new(body_entity, joint));
    //commands
    //.entity(body_entity)
    //.insert(ImpulseJoint::new(avatar, joint));

    commands
}

pub struct DebugEvent {
    id: String,
    translation: Vec3,
}

impl DebugEvent {
    pub fn new(id: &str, translation: &Vec3) -> DebugEvent {
        DebugEvent {
            id: id.to_string(),
            translation: translation.clone(),
        }
    }
}

pub fn ball_avatar_sytem(
    mut writer: EventWriter<DebugEvent>,
    ball_vehicle_query: Query<(&BallVehicle, &Transform), Without<BallVehicleAvatar>>,
    mut ball_vehicle_avatar_query: Query<
        (&BallVehicleAvatar, &mut Transform),
        Without<BallVehicle>,
    >,
) {
    let mut transform_map = HashMap::new();
    for (ball_vehicle, transform) in ball_vehicle_query.iter() {
        transform_map.insert(
            ball_vehicle.combine_id,
            (transform, ball_vehicle.facing_vector(transform.translation)),
        );
    }

    for (ball_vehicle_avatar, mut transform) in ball_vehicle_avatar_query.iter_mut() {
        if let Some((ball_vehicle_transform, forward_vector)) =
            transform_map.get(&ball_vehicle_avatar.combine_id)
        {
            transform.translation = ball_vehicle_transform.translation.clone();

            // calculate rotation from forward vector
            transform.look_at(*forward_vector, Vec3::Y);

            writer.send(DebugEvent::new("6969", forward_vector));
            info!("look at {l}", l = forward_vector);
        }
    }
}

pub fn ball_vehicle_debug_system(
    mut query: Query<(&BallVehicleAvatar, &mut Transform, &GlobalTransform), Without<DebugCube>>,
    mut d_query: Query<(&DebugCube, &mut Transform), Without<BallVehicleAvatar>>,
) {
    for (_, mut transform, global_transform) in query.iter_mut() {
        let (x, y, z) = global_transform
            .compute_transform()
            .rotation
            .to_euler(EulerRot::XYZ);

        // correction angle. we want -pi.

        let correction = -x;

        transform.rotate_x(correction);
        //transform.rotation = Quat::IDENTITY;
        //info!(
        //    "Level -> correction: {c} global angle: {a}",
        //    c = correction,
        //    a = z
        //);
        //info!("X {x:.2} Y {y:.2} Z {z:.2}");

        let v = transform.forward() * 10.0;
        //info!("D: {v}");
        d_query.single_mut().1.translation = v;
    }
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
    let body_density = 10.0;

    let wheel_restitution = 0.0;
    let wheel_friction = 1.8;
    let wheel_density = 8.0;

    let wheel_width = 0.2;

    let max_wheel_force = f32::MAX;
    let max_steer_force = f32::MAX;
    let wheel_factor = 0.7;

    let steering_motor_stiffness = 0.5;
    let steering_motor_damping = 0.5;

    let physics = RigidBody::Dynamic;

    let center_of_mass = Vec3::new(0.0, -3.0, -7.0);
    let ballast_mass = 500.0;

    let body_gltf: Handle<Scene> = asset_server.load("combine-body.glb#Scene0");
    let wheel_gltf: Handle<Scene> = asset_server.load("basic-wheel.glb#Scene0");

    let mut body_commands = commands.spawn(SpatialBundle::from(spawn_transform));

    body_commands
        .insert(Restitution::coefficient(body_restitution))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Friction::coefficient(body_friction))
        .insert(Combine::new(combine_id))
        .insert(physics)
        .insert(Collider::cuboid(3.8, 4.0, 9.0))
        .insert(ColliderMassProperties::Density(body_density))
        .insert(AdditionalMassProperties::MassProperties(MassProperties {
            local_center_of_mass: center_of_mass,
            ..Default::default()
        }))
        .insert(Damping {
            linear_damping: body_linear_damping,
            angular_damping: body_angular_damping,
        })
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: body_gltf,
                transform: Transform::from_xyz(0.0, -3.1, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });

            parent
                .spawn(Collider::cuboid(0.1, 0.1, 0.1))
                .insert(Transform::from_translation(center_of_mass))
                .insert(ColliderMassProperties::Density(ballast_mass));

            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 20.0, 40.0)
                        .with_rotation(Quat::from_rotation_x(-0.4)),
                    camera: Camera {
                        is_active: active_camera,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CombineCamera { combine_id });
        });

    if combine_id != PLAYER_COMBINE_ID {
        body_commands.insert(AiState {
            combine_id,
            ..default()
        });
    }

    let body_entity = body_commands.id();

    let wheel_0_entity = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .with_children(|parent| {
            parent
                .spawn(Wheel)
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::cylinder(wheel_width, 2.0))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

            parent.spawn(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_1_entity = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .with_children(|parent| {
            parent
                .spawn(Wheel)
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::cylinder(wheel_width, 2.0))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

            parent.spawn(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_2_entity = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .with_children(|parent| {
            parent
                .spawn(Wheel)
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::cylinder(wheel_width, 2.0))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

            parent.spawn(SceneBundle {
                scene: wheel_gltf.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let wheel_3_entity = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(physics)
        .with_children(|parent| {
            parent
                .spawn(Wheel)
                .insert(Transform::from_rotation(Quat::from_rotation_z(
                    90.0_f32.to_radians(),
                )))
                .insert(Collider::cylinder(wheel_width, 2.0))
                .insert(Restitution::coefficient(wheel_restitution))
                .insert(Friction::coefficient(wheel_friction))
                .insert(ColliderMassProperties::Density(wheel_density));

            parent.spawn(SceneBundle {
                scene: wheel_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        })
        .id();

    let steering_rack_left = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(physics)
        .id();

    let steering_rack_right = commands
        .spawn(SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(physics)
        .id();

    let x_shift_1 = 4.5;
    let y_shift_1 = 4.0;
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
            .motor_max_force(max_wheel_force),
        RevoluteJointBuilder::new(x)
            .local_anchor1(Vec3::new(-1.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(x_shift_2, y_shift_2, z_shift_2))
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
        .insert(DrivingWheel::new(combine_id));

    commands
        .entity(wheel_1_entity)
        .insert(MultibodyJoint::new(body_entity, revs[1]))
        .insert(DrivingWheel::new(combine_id));

    commands
        .entity(wheel_2_entity)
        .insert(MultibodyJoint::new(steering_rack_left, revs[2]));

    commands
        .entity(wheel_3_entity)
        .insert(MultibodyJoint::new(steering_rack_right, revs[3]));

    commands
}

pub fn combine_speedometer_system(
    mut combine_query: Query<(&mut Combine, &Transform)>,
    time: Res<Time>,
) {
    let time_since_startup = time.elapsed();

    for (mut combine, transform) in combine_query.iter_mut() {
        let vec_length = transform.translation;
        combine
            .velocity_queue
            .push_back((vec_length, time_since_startup));

        if combine.velocity_queue.len() == VELOCITY_QUEUE_LIMIT {
            let (vec_length_comparison, vec_time_comparision) =
                combine.velocity_queue.pop_front().unwrap();

            let vec = ((vec_length.distance(vec_length_comparison))
                / ((time_since_startup - vec_time_comparision).as_millis() as f32))
                * 1000.0;
            combine.velocity = vec;
        }
    }
}

pub fn transmission_system(
    combine_query: Query<&Combine>,
    time: Res<Time>,
    mut drive_wheel_query: Query<&mut DrivingWheel>,
) {
    let mut combine_map: HashMap<i32, f32> = HashMap::new();

    for combine in combine_query.iter() {
        let factor = 1.33;
        let constant = 3.0;

        /*let mut velocity = 2.0;
        if combine.velocity > 30.0 {
            velocity = 40.0;
        } else if combine.velocity > 15.0 {
            velocity = 20.0;
        }*/

        let velocity = (combine.velocity * factor) + constant;

        // compensate for 60fps target

        let adj_velocity = ((time.delta().as_millis() as f32) / (1000.0 / 60.0)) * velocity;

        combine_map.insert(combine.combine_id, adj_velocity);
    }

    for mut drive_wheel in drive_wheel_query.iter_mut() {
        let velocity = combine_map.get(&drive_wheel.combine_id).unwrap();
        drive_wheel.target_velocity = *velocity;
    }
}
