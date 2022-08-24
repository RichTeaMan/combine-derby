use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    camera,
    combine::{Combine, PLAYER_COMBINE_ID},
    config,
};

#[derive(Component)]
pub struct TextChanges;

#[derive(Component)]
pub struct SpeedoUi;

#[derive(Component)]
pub struct DescriptionUi;

#[derive(Component)]
pub struct DebugUi;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mono_font = asset_server.load("fonts/FiraMono-Regular.ttf");
    let regular_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: mono_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(TextChanges);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(30.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: regular_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(SpeedoUi);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(330.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: regular_font,
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(DescriptionUi);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: mono_font,
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(DebugUi);
}

pub fn change_text_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        text.sections[0].value = format!(
            "Build {}, {} {:.1} fps",
            config::GIT_VERSION,
            config::BUILD_DATE,
            fps
        );
    }
}

pub fn combine_ui_system(
    combine_query: Query<&Combine>,
    mut text_query: Query<&mut Text, With<SpeedoUi>>,
) {
    let mut text = text_query.single_mut();

    for combine in combine_query.iter() {
        if combine.combine_id == PLAYER_COMBINE_ID {
            text.sections[0].value = format!("{:.0} m/s", combine.velocity);
        }
    }
}

pub fn update_debug_ui_system(
    windows: Res<Windows>,
    mut query: Query<&mut Text, With<DebugUi>>,
    mut style_query: Query<&mut Style, With<DebugUi>>,
) {
    for mut text in query.iter_mut() {
        let resolution = camera::get_primary_window_size(&windows);

        text.sections[0].value = format!("Resolution:{}, {}", resolution.x, resolution.y);
    }
    for mut text in style_query.iter_mut() {
        text.position = UiRect {
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        };
    }
}
