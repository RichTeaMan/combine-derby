// shamelessly taken from the Bevy cheatbook
// https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub struct PlayerCamera;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 250.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    // orbit camera with right mouse OR with left mouse and ctrl key pressed.
    if input_mouse.pressed(orbit_button)
        || (input_mouse.pressed(pan_button)
            && (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl)))
    {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis

            // stops camera tilting below horizon.
            if (transform.rotation.to_euler(EulerRot::YXZ)).1 > 0.0 {
                transform.rotation *= Quat::from_rotation_x(-f32::abs(delta_y));
            }
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // don't allow zoom too far.
            pan_orbit.radius = f32::max(pan_orbit.radius, 20.0);

            // don't allow zoom too far out.
            pan_orbit.radius = f32::min(pan_orbit.radius, 2500.0);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

pub fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();

    Vec2::new(window.width() as f32, window.height() as f32)
}

/// Spawn a camera like this
pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-12.0, 12.5, 5.0);

    let pan_orbit = PanOrbitCamera {
        ..Default::default()
    };

    let mut start_transform =
        Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y);
    let rot_matrix = Mat3::from_quat(start_transform.rotation);
    start_transform.translation =
        pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));

    commands
        .spawn_bundle(Camera3dBundle {
            transform: start_transform,
            ..Default::default()
        })
        .insert(pan_orbit)
        .insert(PlayerCamera);
}
