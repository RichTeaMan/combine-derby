use bevy::prelude::*;

use crate::{
    camera::SwitchCameraEvent,
    events::{Control, ControlEvent},
};

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut control_events: ResMut<Events<ControlEvent>>,
    mut camera_events: ResMut<Events<SwitchCameraEvent>>,
) {
    if keys.pressed(KeyCode::W) {
        control_events.send(ControlEvent {
            control: Control::Forward,
        });
    } else if keys.pressed(KeyCode::S) {
        control_events.send(ControlEvent {
            control: Control::Back,
        });
    } else {
        control_events.send(ControlEvent {
            control: Control::NoPower,
        });
    }

    if keys.pressed(KeyCode::A) {
        control_events.send(ControlEvent {
            control: Control::Left,
        });
    } else if keys.pressed(KeyCode::D) {
        control_events.send(ControlEvent {
            control: Control::Right,
        });
    } else {
        control_events.send(ControlEvent {
            control: Control::NoSteer,
        })
    }

    if keys.just_pressed(KeyCode::F4) {
        camera_events.send(SwitchCameraEvent);
    }

    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
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
