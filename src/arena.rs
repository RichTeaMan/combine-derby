use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const PLANE_SIZE: f32 = 200.0;
pub const RAMP_HEIGHT: f32 = 5.0;

pub fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const TILE_SIZE: f32 = 20.0;
    const FENCE_HEIGHT: f32 = 20.0;
    let ground_plane_handle = meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE }));
    let fence_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE * 4.0,
        },
        flip: false,
    }));

    let ground_texture = asset_server.load("ground_texture.png");
    let ground_normal_texture: Handle<Image> = asset_server.load("ground_normal_texture.png");

    let fence_texture = asset_server.load("fence.png");

    let ground_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.31, 0.21, 0.14),
        base_color_texture: Some(ground_texture),
        normal_map_texture: Some(ground_normal_texture),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });

    let fence_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(fence_texture),
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
        )))
        .insert(Friction::coefficient(0.8));

    // actual fence height is enormous, bigger than the mesh.
    commands
        .spawn()
        .insert(Collider::cuboid(0.1, FENCE_HEIGHT * 10.0, PLANE_SIZE))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            PLANE_SIZE,
            FENCE_HEIGHT * 10.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(0.1, FENCE_HEIGHT * 10.0, PLANE_SIZE))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -PLANE_SIZE,
            FENCE_HEIGHT * 10.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(PLANE_SIZE, FENCE_HEIGHT * 10.0, 0.1))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            FENCE_HEIGHT * 10.0,
            PLANE_SIZE,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(PLANE_SIZE, FENCE_HEIGHT * 10.0, 0.1))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            FENCE_HEIGHT * 10.0,
            -PLANE_SIZE,
        )));

    let tile_count = (PLANE_SIZE / TILE_SIZE) as i32;
    for i in -tile_count..=tile_count {
        for j in -tile_count..=tile_count {
            commands.spawn_bundle(PbrBundle {
                mesh: ground_plane_handle.clone(),
                material: ground_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 * TILE_SIZE) as f32,
                    ground_y_position,
                    (j as f32 * TILE_SIZE) as f32,
                )),
                ..default()
            });
        }

        // setup arena fences
        commands.spawn_bundle(PbrBundle {
            mesh: fence_handle.clone(),
            material: fence_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                (i as f32 * TILE_SIZE) as f32,
                ground_y_position + (TILE_SIZE / 2.0),
                -PLANE_SIZE,
            )),
            ..default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: fence_handle.clone(),
            material: fence_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                -(i as f32 * TILE_SIZE) as f32,
                ground_y_position + (TILE_SIZE / 2.0),
                PLANE_SIZE,
            ))
            .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
            ..default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: fence_handle.clone(),
            material: fence_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                -PLANE_SIZE,
                ground_y_position + (TILE_SIZE / 2.0),
                (i as f32 * TILE_SIZE) as f32,
            ))
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
            ..default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: fence_handle.clone(),
            material: fence_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                PLANE_SIZE,
                ground_y_position + (TILE_SIZE / 2.0),
                -(i as f32 * TILE_SIZE) as f32,
            ))
            .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
            ..default()
        });
    }

    // setup ramps

    let half_ramp_height = RAMP_HEIGHT / 2.0;
    let ramp_length = PLANE_SIZE / 2.0;
    let slope_rotation = (RAMP_HEIGHT / ramp_length).atan();

    let ramp_plane_handle = meshes.add(Mesh::from(shape::Plane {
        size: PLANE_SIZE / 2.0,
    }));
    let ramp_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.66, 0.66, 0.68),
        perceptual_roughness: 0.1,
        unlit: true,
        ..default()
    });

    // ramp 1

    // plinth
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            PLANE_SIZE - (PLANE_SIZE / 4.0),
            ground_y_position + half_ramp_height,
            PLANE_SIZE - (PLANE_SIZE / 4.0),
        )))
        .insert(Collider::cuboid(
            PLANE_SIZE / 4.0,
            half_ramp_height,
            PLANE_SIZE / 4.0,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, half_ramp_height, 0.0)),
                ..default()
            });
        });

    // slope
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE - (PLANE_SIZE / 2.0),
            )
            .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE - (PLANE_SIZE / 2.0),
            )
            .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -0.25 * PLANE_SIZE,
                    half_ramp_height,
                    0.25 * PLANE_SIZE,
                ))
                .with_rotation(Quat::from_rotation_x(slope_rotation)),
                ..default()
            });
        });
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE - (PLANE_SIZE / 2.0),
            )
            .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(-PLANE_SIZE / 2.0, 0.0, ramp_length),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

    // slope
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE - (PLANE_SIZE / 2.0),
            )
            .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE,
            )
            .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -0.25 * PLANE_SIZE,
                    half_ramp_height,
                    0.25 * PLANE_SIZE,
                ))
                .with_rotation(Quat::from_rotation_x(slope_rotation)),
                ..default()
            });
        });
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                PLANE_SIZE - (PLANE_SIZE / 2.0),
                ground_y_position,
                PLANE_SIZE,
            )
            .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(-PLANE_SIZE / 2.0, 0.0, ramp_length),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

    // ramp 2

    // plinth
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            -(PLANE_SIZE - (PLANE_SIZE / 4.0)),
            ground_y_position + half_ramp_height,
            -(PLANE_SIZE - (PLANE_SIZE / 4.0)),
        )))
        .insert(Collider::cuboid(
            PLANE_SIZE / 4.0,
            half_ramp_height,
            PLANE_SIZE / 4.0,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, half_ramp_height, 0.0)),
                ..default()
            });
        });

    // slope
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
            )
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -PLANE_SIZE,
            )
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -0.25 * PLANE_SIZE,
                    half_ramp_height,
                    0.25 * PLANE_SIZE,
                ))
                .with_rotation(Quat::from_rotation_x(slope_rotation)),
                ..default()
            });
        });
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -PLANE_SIZE,
            )
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(-PLANE_SIZE / 2.0, 0.0, ramp_length),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

    // slope
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
            )
            .with_rotation(Quat::from_rotation_y(0.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
            )
            .with_rotation(Quat::from_rotation_y(0.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(0.0, RAMP_HEIGHT, 0.0),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: ramp_plane_handle.clone(),
                material: ramp_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -0.25 * PLANE_SIZE,
                    half_ramp_height,
                    0.25 * PLANE_SIZE,
                ))
                .with_rotation(Quat::from_rotation_x(slope_rotation)),
                ..default()
            });
        });
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
                ground_y_position,
                -(PLANE_SIZE - (PLANE_SIZE / 2.0)),
            )
            .with_rotation(Quat::from_rotation_y(0.0_f32.to_radians())),
        ))
        .insert(Collider::triangle(
            Vec3::new(-PLANE_SIZE / 2.0, 0.0, ramp_length),
            Vec3::new(-PLANE_SIZE / 2.0, RAMP_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, ramp_length),
        ));

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
}
