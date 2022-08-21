use bevy::prelude::*;

use crate::events::{Control, ControlEvent};

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut events: ResMut<Events<ControlEvent>>) {
    if keys.pressed(KeyCode::W) {
        events.send(ControlEvent {
            control: Control::Forward,
        });
    } else if keys.pressed(KeyCode::S) {
        events.send(ControlEvent {
            control: Control::Back,
        });
    } else {
        events.send(ControlEvent {
            control: Control::NoPower,
        });
    }

    if keys.pressed(KeyCode::A) {
        events.send(ControlEvent {
            control: Control::Left,
        });
    } else if keys.pressed(KeyCode::D) {
        events.send(ControlEvent {
            control: Control::Right,
        });
    } else {
        events.send(ControlEvent {
            control: Control::NoSteer,
        })
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
