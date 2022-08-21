use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::{
    camera::PanOrbitCamera,
    combine::{CameraTarget, Combine},
    events::{Control, ControlEvent},
    DebugPointer,
};

pub fn control_events(
    mut events: EventReader<ControlEvent>,
    mut selected_query: Query<(&mut ExternalForce, &Combine, &Transform)>,
) {
    for event in events.iter() {
        for q in selected_query.iter_mut() {
            let combine = q.1;
            if !combine.player_controlled {
                continue;
            }
            let mut external_force = q.0;
            let transform = q.2;

            match event.control {
                Control::Forward => {
                    let force = transform.forward() * combine.engine_force;
                    external_force.force = force;
                }
                Control::Back => {
                    external_force.force = transform.back() * combine.reverse_engine_force;
                    //     info!("Applying back force {f}", f = external_force.force);
                }
                Control::NoPower => {
                    external_force.force = Vec3::new(0.0, 0.0, 0.0);
                    // info!("no force");
                }

                Control::Left => {
                    external_force.torque = Vec3::new(0.0, combine.steering_force, 0.0);
                    info!("left");
                }
                Control::Right => {
                    external_force.torque = Vec3::new(0.0, -combine.steering_force, 0.0);
                }
                Control::NoSteer => {
                    external_force.torque = Vec3::new(0.0, 0.0, 0.0);
                }
            }
        }
    }
}
