use bevy::prelude::*;

pub enum Control {
    Forward,
    Left,
    NoSteer,
    Right,
    Back,
    NoPower
}
pub struct ControlEvent {
    pub control: Control
}

