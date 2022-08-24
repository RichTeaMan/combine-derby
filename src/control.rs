use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

use crate::{
    combine::{DrivingWheel, SteeringWheel},
    events::{SpeedControlAction, SpeedControlEvent, SteerControlAction, SteerControlEvent},
};

pub fn speed_control_events(
    mut speed_control_events: EventReader<SpeedControlEvent>,
    mut query: Query<(&DrivingWheel, &mut MultibodyJoint)>,
) {
    let target_velocity = 50.0;
    let factor = 1.0;

    let mut control_map = HashMap::new();
    for event in speed_control_events.iter() {
        control_map.insert(event.combine_id, event.action.clone());
    }

    for mut q in query.iter_mut() {
        if let Some(action) = control_map.get(&q.0.combine_id) {
            match action {
                SpeedControlAction::Forward => {
                    q.1.data
                        .set_motor_velocity(JointAxis::AngX, -target_velocity, factor);
                    q.1.data.set_limits(JointAxis::AngX, [f32::MIN, f32::MAX]);
                }
                SpeedControlAction::Back => {
                    q.1.data
                        .set_motor_velocity(JointAxis::AngX, target_velocity / 2.0, factor);
                    q.1.data.set_limits(JointAxis::AngX, [f32::MIN, f32::MAX]);
                }
                SpeedControlAction::NoPower => {
                    q.1.data.set_motor_velocity(JointAxis::AngX, 0.0, factor);
                }
                SpeedControlAction::Brake => {
                    q.1.data.set_motor_velocity(JointAxis::AngX, 0.0, factor);
                    q.1.data.set_limits(JointAxis::AngX, [0.0, 0.0]);
                }
            }
        }
    }
}

pub fn steer_control_events(
    mut steer_control_events: EventReader<SteerControlEvent>,
    mut query: Query<(&SteeringWheel, &mut MultibodyJoint)>,
) {
    let mut control_map = HashMap::new();

    let angle = 35.0_f32.to_radians();
    let straight = 0.0_f32.to_radians();

    for event in steer_control_events.iter() {
        control_map.insert(event.combine_id, event.action.clone());
    }

    for mut q in query.iter_mut() {
        if let Some(action) = control_map.get(&q.0.combine_id) {
            match action {
                SteerControlAction::Left => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, -angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [-angle, straight]);
                }
                SteerControlAction::NoSteer => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, straight, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [straight, straight]);
                }
                SteerControlAction::Right => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [straight, angle]);
                }
            }
        }
    }
}
