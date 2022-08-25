use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

use crate::{
    combine::{DrivingWheel, SteeringWheel, SteeringWheelPosition},
    events::{SpeedControlAction, SpeedControlEvent, SteerControlAction, SteerControlEvent},
};

pub fn speed_control_events(
    mut speed_control_events: EventReader<SpeedControlEvent>,
    mut query: Query<(&DrivingWheel, &mut MultibodyJoint)>,
) {
    let factor = 0.1;

    let mut control_map = HashMap::new();
    for event in speed_control_events.iter() {
        control_map.insert(event.combine_id, event.action.clone());
    }

    for (driving_wheel, mut joint) in query.iter_mut() {
        let target_velocity = driving_wheel.target_velocity;
        if let Some(action) = control_map.get(&driving_wheel.combine_id) {
            match action {
                SpeedControlAction::Forward => {
                    joint
                        .data
                        .set_motor_velocity(JointAxis::AngX, -target_velocity, factor);
                    joint.data.set_limits(JointAxis::AngX, [f32::MIN, f32::MAX]);
                }
                SpeedControlAction::Back => {
                    joint
                        .data
                        .set_motor_velocity(JointAxis::AngX, target_velocity / 2.0, factor);
                    joint.data.set_limits(JointAxis::AngX, [f32::MIN, f32::MAX]);
                }
                SpeedControlAction::NoPower => {
                    joint.data.set_motor_velocity(JointAxis::AngX, 0.0, factor);
                }
                SpeedControlAction::Brake => {
                    joint.data.set_motor_velocity(JointAxis::AngX, 0.0, factor);
                    joint.data.set_limits(JointAxis::AngX, [0.0, 0.0]);
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

    let angle = 15.0_f32.to_radians();
    let straight = 0.0_f32.to_radians();

    for event in steer_control_events.iter() {
        control_map.insert(event.combine_id, event.action.clone());
    }

    for (steering_wheel, mut joint) in query.iter_mut() {
        if let Some(action) = control_map.get(&steering_wheel.combine_id) {
            let mut adjusted_angle = angle;
            if steering_wheel.steering_wheel_position == SteeringWheelPosition::Left {
                adjusted_angle =
                    calc_left_angle(steering_wheel.combine_wheel_base, steering_wheel.combine_track_width, angle);
            } else if steering_wheel.steering_wheel_position == SteeringWheelPosition::Right {
                adjusted_angle =
                    calc_right_angle(steering_wheel.combine_wheel_base, steering_wheel.combine_track_width, angle);
            }
            match action {
                SteerControlAction::Left => {
                    joint.data
                        .set_motor_position(JointAxis::AngX, -adjusted_angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [-adjusted_angle, -adjusted_angle]);
                }
                SteerControlAction::NoSteer => {
                    joint.data
                        .set_motor_position(JointAxis::AngX, straight, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [straight, straight]);
                }
                SteerControlAction::Right => {
                    joint.data
                        .set_motor_position(JointAxis::AngX, adjusted_angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [adjusted_angle, adjusted_angle]);
                }
            }
        }
    }
}

/**
 * Gets the right wheel angle given the relevant steering angle.
 *
 * Angles are in radians.
 */
fn calc_right_angle(wheel_base: f32, track_width: f32, steering_angle: f32) -> f32 {
    let tan_steering = steering_angle.tan();
    ((tan_steering * wheel_base) / (wheel_base + (0.5 * track_width * tan_steering))).atan()
}

/**
 * Gets the left wheel angle given the relevant steering angle.
 *
 * Angles are in radians.
 */
fn calc_left_angle(wheel_base: f32, track_width: f32, steering_angle: f32) -> f32 {
    let tan_steering = steering_angle.tan();
    ((tan_steering * wheel_base) / (wheel_base - (0.5 * track_width * tan_steering))).atan()
}

#[cfg(test)]
mod tests {
    use crate::control::{calc_left_angle, calc_right_angle};

    #[test]
    fn left_angle_test() {
        let angle: f32 = 35.0;
        let wheel_angle = calc_left_angle(12.0, 12.0, angle.to_radians()).to_degrees();

        assert_eq!(47.13413, wheel_angle);
    }

    #[test]
    fn right_angle_test() {
        let angle: f32 = 35.0;
        let wheel_angle = calc_right_angle(12.0, 12.0, angle.to_radians()).to_degrees();

        assert_eq!(27.412718, wheel_angle);
    }

    #[test]
    fn left_angle_neg_test() {
        let angle: f32 = -35.0;
        let wheel_angle = calc_left_angle(12.0, 12.0, angle.to_radians()).to_degrees();

        assert_eq!(-27.412718, wheel_angle);
    }

    #[test]
    fn right_angle_neg_test() {
        let angle: f32 = -35.0;
        let wheel_angle = calc_right_angle(12.0, 12.0, angle.to_radians()).to_degrees();

        assert_eq!(-47.13413, wheel_angle);
    }
}
