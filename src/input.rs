use bevy::prelude::*;

use crate::{
    camera::SwitchCameraEvent,
    events::{SpeedControlEvent, SteerControlEvent},
};

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut speed_control_events: ResMut<Events<SpeedControlEvent>>,
    mut steer_control_events: ResMut<Events<SteerControlEvent>>,
    mut camera_events: ResMut<Events<SwitchCameraEvent>>,
) {
    if keys.pressed(KeyCode::W) {
        speed_control_events.send(SpeedControlEvent::Forward);
    } else if keys.pressed(KeyCode::S) {
        speed_control_events.send(SpeedControlEvent::Back);
    } else {
        speed_control_events.send(SpeedControlEvent::NoPower);
    }

    if keys.pressed(KeyCode::A) {
        steer_control_events.send(SteerControlEvent::Left);
    } else if keys.pressed(KeyCode::D) {
        steer_control_events.send(SteerControlEvent::Right);
    } else {
        steer_control_events.send(SteerControlEvent::NoSteer);
    }

    if keys.just_pressed(KeyCode::F4) {
        camera_events.send(SwitchCameraEvent);
    }

    if keys.pressed(KeyCode::Space) {
        speed_control_events.send(SpeedControlEvent::Brake);
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
