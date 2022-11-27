use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    arena::PLANE_SIZE,
    combine::{Combine, Wheel, PLAYER_COMBINE_ID},
    events::SoundSampleEvent,
    sounds::SoundCollider,
};

const HAY_BALE_DIMENSION: f32 = 9.5;

#[derive(Component)]
pub struct Cow;

fn spawn_hay_bale_with_transform<'w, 's>(
    mut commands: Commands<'w, 's>,
    transform: Transform,
    scene_handle: Handle<Scene>,
) -> Commands<'w, 's> {
    commands
        .spawn(SpatialBundle::from(transform))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(3.72, 4.5))
        .insert(Restitution::coefficient(0.7))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(SoundCollider {
            sound_sample: SoundSampleEvent::HayBale,
        })
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: scene_handle,
                transform: Transform::from_xyz(4.5, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(6.0, 6.0, 6.0)),
                ..Default::default()
            });
        });

    commands
}

pub fn spawn_hay_bales(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bale_gltf: Handle<Scene> = asset_server.load("hay-bale.glb#Scene0");

    let layer_count = 4;
    let layer_height = 8.0;
    let layer_start_height = 1.5;
    // the hay pyramid
    for layer in 0..layer_count {
        let current_width = layer_count - layer;
        let offset = -current_width as f32 * 0.5 * HAY_BALE_DIMENSION;
        for x in 0..current_width {
            for y in 0..current_width {
                let transform = Transform::from_translation(Vec3::new(
                    offset + ((x as f32) * HAY_BALE_DIMENSION),
                    (layer_height * layer as f32) + layer_start_height,
                    offset + ((y as f32) * HAY_BALE_DIMENSION),
                ));
                commands = spawn_hay_bale_with_transform(commands, transform, bale_gltf.clone());
            }
        }
    }
}

pub fn spawn_cows(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cow_gltf: Handle<Scene> = asset_server.load("cow.glb#Scene0");

    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(150.0, 12.0, 50.0)),
        cow_gltf.clone(),
    );
    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(50.0, 12.0, 150.0)),
        cow_gltf.clone(),
    );
    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(-200.0, 12.0, -100.0)),
        cow_gltf.clone(),
    );
    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(-50.0, 12.0, -150.0)),
        cow_gltf.clone(),
    );

    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(PLANE_SIZE - 100.0, 12.0, -100.0)),
        cow_gltf.clone(),
    );

    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(-100.0, 12.0, PLANE_SIZE - 100.0)),
        cow_gltf.clone(),
    );

    commands = spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(-(PLANE_SIZE - 100.0), 12.0, 100.0)),
        cow_gltf.clone(),
    );

    spawn_cow_with_transform(
        commands,
        Transform::from_translation(Vec3::new(100.0, 12.0, -(PLANE_SIZE - 100.0))),
        cow_gltf,
    );
}

pub fn cow_ai_system(
    mut cow_query: Query<&mut Transform, With<Cow>>,
    combine_query: Query<(&Combine, &Transform), Without<Cow>>,
) {
    let mut player_combine_opt = Option::None;

    for (combine, transform) in combine_query.iter() {
        if combine.combine_id == PLAYER_COMBINE_ID {
            player_combine_opt = Some(transform);
            break;
        }
    }

    for mut cow_transform in cow_query.iter_mut() {
        cow_transform.look_at(player_combine_opt.unwrap().translation, Vec3::Y);
    }
}

fn spawn_cow_with_transform<'w, 's>(
    mut commands: Commands<'w, 's>,
    transform: Transform,
    scene_handle: Handle<Scene>,
) -> Commands<'w, 's> {
    commands
        .spawn(SpatialBundle::from(transform))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(8.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ColliderMassProperties::Density(0.5))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(Cow)
        .insert(SoundCollider {
            sound_sample: SoundSampleEvent::Cow,
        })
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: scene_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians()))
                    .with_scale(Vec3::new(4.0, 4.0, 4.0)),
                ..Default::default()
            });
        });

    commands
}

pub fn collision_check_system(
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut sound_samples_events: EventWriter<SoundSampleEvent>,
    sound_collider_query: Query<&SoundCollider>,
    combine_query: Query<&Combine>,
    wheel_query: Query<&Wheel>,
) {
    for contact_force_event in contact_force_events.iter() {
        let mut hits = 0;
        let mut sound_event_sample = Option::None;
        if let Ok(a) = sound_collider_query.get(contact_force_event.collider1) {
            hits += 1;
            sound_event_sample = Some(a.sound_sample.clone());
        } else if let Ok(b) = sound_collider_query.get(contact_force_event.collider2) {
            hits += 1;
            sound_event_sample = Some(b.sound_sample.clone());
        }

        if combine_query.get(contact_force_event.collider1).is_ok() {
            hits += 1;
        } else if combine_query.get(contact_force_event.collider2).is_ok() {
            hits += 1;
        }
        if wheel_query.get(contact_force_event.collider1).is_ok() {
            hits += 1;
        } else if wheel_query.get(contact_force_event.collider2).is_ok() {
            hits += 1;
        }
        if hits > 1 {
            if let Some(sample) = sound_event_sample {
                sound_samples_events.send(sample);
            }
        }
    }
}
