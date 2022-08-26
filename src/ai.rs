use bevy::prelude::*;

use crate::{
    combine::Combine,
    events::{SpeedControlEvent, SteerControlEvent},
};

#[derive(Component)]
pub struct AiState {
    pub combine_id: i32,

    pub frames_at_zero_velocity: i32,

    pub reverse_frames: i32,
}

impl Default for AiState {
    fn default() -> Self {
        AiState {
            combine_id: 0,
            frames_at_zero_velocity: 0,
            reverse_frames: -1,
        }
    }
}

pub fn combine_ai_system(
    mut combine_ai_query: Query<(&mut AiState, &Combine)>,
    mut speed_control_events: ResMut<Events<SpeedControlEvent>>,
    mut steer_control_events: ResMut<Events<SteerControlEvent>>,
) {
    for (mut ai, combine) in combine_ai_query.iter_mut() {
        if combine.velocity < 0.1 {
            ai.frames_at_zero_velocity = ai.frames_at_zero_velocity + 1;
        } else {
            ai.frames_at_zero_velocity = 0;
        }

        if ai.frames_at_zero_velocity > 100 {
            ai.reverse_frames = 100;
        }

        if ai.reverse_frames != -1 {
            speed_control_events.send(SpeedControlEvent {
                combine_id: combine.combine_id,
                action: crate::events::SpeedControlAction::Back,
            });
            steer_control_events.send(SteerControlEvent {
                combine_id: combine.combine_id,
                action: crate::events::SteerControlAction::Left,
            });
        } else {
            speed_control_events.send(SpeedControlEvent {
                combine_id: combine.combine_id,
                action: crate::events::SpeedControlAction::Forward,
            });
        }
    }
}
