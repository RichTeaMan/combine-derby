use bevy::prelude::*;

use crate::{
    camera::SwitchCameraEvent,
    combine::PLAYER_COMBINE_ID,
    events::{SpeedControlAction, SpeedControlEvent, SteerControlAction, SteerControlEvent},
    ui::DebugInfo, config::DEFAULT_VOLUME,
};

#[derive(Resource)]
pub struct Settings {
    pub volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { volume: DEFAULT_VOLUME }
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut speed_control_events: ResMut<Events<SpeedControlEvent>>,
    mut steer_control_events: ResMut<Events<SteerControlEvent>>,
    mut camera_events: ResMut<Events<SwitchCameraEvent>>,
    mut debug_info: ResMut<DebugInfo>,
    mut settings: ResMut<Settings>,
) {
    let combine_id = PLAYER_COMBINE_ID;

    if keys.pressed(KeyCode::W) {
        speed_control_events.send(SpeedControlEvent {
            combine_id,
            action: SpeedControlAction::Forward,
        });
    } else if keys.pressed(KeyCode::S) {
        speed_control_events.send(SpeedControlEvent {
            combine_id,
            action: SpeedControlAction::Back,
        });
    } else {
        speed_control_events.send(SpeedControlEvent {
            combine_id,
            action: SpeedControlAction::NoPower,
        });
    }

    if keys.pressed(KeyCode::A) {
        steer_control_events.send(SteerControlEvent {
            combine_id,
            action: SteerControlAction::Left,
        });
    } else if keys.pressed(KeyCode::D) {
        steer_control_events.send(SteerControlEvent {
            combine_id,
            action: SteerControlAction::Right,
        });
    } else {
        steer_control_events.send(SteerControlEvent {
            combine_id,
            action: SteerControlAction::NoSteer,
        });
    }

    if keys.just_pressed(KeyCode::F4) {
        camera_events.send(SwitchCameraEvent);
    }
    if keys.just_pressed(KeyCode::F3) {
        debug_info.enabled = !debug_info.enabled;
    }
    if keys.just_pressed(KeyCode::F6) {
        settings.volume = if settings.volume > 0.0 { 0.0 } else { 1.0 };
    }
}
