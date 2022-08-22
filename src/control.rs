use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

use crate::{
    combine::{DrivingWheel, SteeringWheel, SteeringWheelPosition},
    events::{SpeedControlEvent, SteerControlEvent},
};


pub fn speed_control_events(
    mut speed_control_events: EventReader<SpeedControlEvent>,
    mut query: Query<(&DrivingWheel, &mut MultibodyJoint)>,
) {
    let target_velocity = 30.0;
    let factor = 0.5;

    for event in speed_control_events.iter() {
        for mut q in query.iter_mut() {
            match event {
                SpeedControlEvent::Forward => {
                    q.1.data.set_motor_velocity(JointAxis::AngX, -target_velocity, factor);
                }
                SpeedControlEvent::Back => {
                    q.1.data.set_motor_velocity(JointAxis::AngX, target_velocity, factor);
                }
                SpeedControlEvent::NoPower => {
                    q.1.data.set_motor_velocity(JointAxis::AngX, 0.0, factor);
                }
            }
        }
    }
}

pub fn steer_control_events(
    mut steer_control_events: EventReader<SteerControlEvent>,
    mut query: Query<(&SteeringWheel, &mut MultibodyJoint)>,
) {
    for event in steer_control_events.iter() {
        for mut q in query.iter_mut() {
            let mut straight = 10.0_f32.to_radians();
            if q.0.steering_wheel_position == SteeringWheelPosition::Right {
                straight = -straight;
            }

            let angle = 30.0_f32.to_radians();

            match event {
                SteerControlEvent::Left => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, angle, 1.0, 0.5);
                }
                SteerControlEvent::NoSteer => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, straight, 1.0, 0.1);
                }
                SteerControlEvent::Right => {
                    q.1.data
                        .set_motor_position(JointAxis::AngX, -angle, 1.0, 0.5);
                }
            }
        }
    }
}
