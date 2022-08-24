mod camera;
mod combine;
mod config;
mod control;
mod events;
mod input;
mod obstacle;
mod ui;
mod sounds;
mod arena;

use arena::setup_arena;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier3d::prelude::*;
use camera::{camera_events, SwitchCameraEvent};
use combine::{spawn_combines, combine_speedometer_system, transmission_system};

use control::{speed_control_events, steer_control_events};
use events::{SpeedControlEvent, SteerControlEvent, SoundSampleEvent};
use input::keyboard_input;

use obstacle::{spawn_hay_bales, spawn_cows, collision_check_system};
use sounds::{play_sample, setup_sounds};
use ui::{change_text_system, infotext_system, update_debug_ui_system, combine_ui_system};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_event::<SpeedControlEvent>()
        .add_event::<SteerControlEvent>()
        .add_event::<SwitchCameraEvent>()
        .add_event::<SoundSampleEvent>()
        .add_startup_system(setup_arena)
        .add_startup_system(setup_sounds)
        .add_startup_system(spawn_combines)
        .add_startup_system(camera::spawn_camera)
        .add_startup_system(infotext_system)
        .add_startup_system(spawn_hay_bales)
        .add_startup_system(spawn_cows)
        .add_system(camera::pan_orbit_camera)
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_input)
        .add_system(speed_control_events)
        .add_system(steer_control_events)
        .add_system(camera_events)
        .add_system(update_debug_ui_system)
        .add_system(change_text_system)
        .add_system(collision_check_system)
        .add_system(play_sample)
        .add_system(combine_ui_system)
        .add_system(combine_speedometer_system)
        .add_system(transmission_system)
        .run();
}
