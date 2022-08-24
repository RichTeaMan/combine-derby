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
            let mut adjusted_angle = angle;
            if q.0.steering_wheel_position == SteeringWheelPosition::Left {
                adjusted_angle =
                    calc_left_angle(q.0.combine_wheel_base, q.0.combine_track_width, angle);
            } else if q.0.steering_wheel_position == SteeringWheelPosition::Right {
                adjusted_angle =
                    calc_right_angle(q.0.combine_wheel_base, q.0.combine_track_width, angle);
            }
            match action {
                SteerControlAction::Left => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, -adjusted_angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [-adjusted_angle, straight]);
                }
                SteerControlAction::NoSteer => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, straight, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [straight, straight]);
                }
                SteerControlAction::Right => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, adjusted_angle, 1.0, 0.5)
                        .set_limits(JointAxis::AngX, [straight, adjusted_angle]);
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
