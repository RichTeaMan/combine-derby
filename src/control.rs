use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

use crate::{
    events::{Control, ControlEvent}, combine::Motor,
};

pub fn control_events(
   mut events: EventReader<ControlEvent>,
    mut query			: Query<&mut ImpulseJoint>,
) {
    for event in events.iter() {
        for mut q in query.iter_mut() {
/*            let combine = q.1;
            if !combine.player_controlled {
                continue;
            }
            let mut external_force = q.0;
            let transform = q.2;
*/
            match event.control {
                Control::Forward => {
                    q.data.set_motor_velocity(JointAxis::AngX, -30.0, 0.0);
                }
                Control::Back => {
                    q.data.set_motor_velocity(JointAxis::AngX, 30.0, 0.0);
                }
                Control::NoPower => {
                    q.data.set_motor_velocity(JointAxis::AngX, 0.0, 0.0);
                }
                Control::Left => todo!(),
                Control::NoSteer => {},
                Control::Right => todo!(),
/*
                Control::Left => {
                    external_force.torque = Vec3::new(0.0, combine.steering_force, 0.0);
                }
                Control::Right => {
                    external_force.torque = Vec3::new(0.0, -combine.steering_force, 0.0);
                }
                Control::NoSteer => {
                    external_force.torque = Vec3::new(0.0, 0.0, 0.0);
                }
                */
            }
        }
    }
}
