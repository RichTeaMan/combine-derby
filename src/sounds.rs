use std::time::Duration;

use bevy::{audio::AudioSink, prelude::*};

use crate::events::SoundSampleEvent;

#[derive(Component)]
pub struct SoundCollider {
    pub sound_sample: SoundSampleEvent,
}

#[derive(Component)]
pub struct SoundSamples {
    pub moo: Handle<AudioSource>,

    pub hay: Handle<AudioSource>,

    pub last_moo_time: Duration,

    pub last_hay_time: Duration,

    pub engine_sound_sink: Handle<AudioSink>,
}

pub fn setup_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let moo: Handle<AudioSource> = asset_server.load("sounds/moo.ogg");
    let hay: Handle<AudioSource> = asset_server.load("sounds/hay1.ogg");
    let engine_sample: Handle<AudioSource> = asset_server.load("sounds/engine_heavy_loop.ogg");

    let background: Handle<AudioSource> = asset_server.load("sounds/jazzyfrenchy.ogg");

    let engine_sound_sink = audio.play_with_settings(
        engine_sample,
        PlaybackSettings {
            repeat: true,
            volume: 0.1,
            ..Default::default()
        },
    );

    audio.play_with_settings(
        background,
        PlaybackSettings {
            repeat: true,
            volume: 0.2,
            ..Default::default()
        },
    );

    commands.spawn(SoundSamples {
        moo,
        hay,
        last_moo_time: Duration::ZERO,
        last_hay_time: Duration::ZERO,
        engine_sound_sink: audio_sinks.get_handle(engine_sound_sink),
    });
}

pub fn play_sample(
    mut sound_sample_events: EventReader<SoundSampleEvent>,
    mut sound_samples_query: Query<&mut SoundSamples>,
    audio: Res<Audio>,
    time: Res<Time>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let mut sound_samples = sound_samples_query.single_mut();

    for sound_sample_event in sound_sample_events.iter() {
        match sound_sample_event {
            SoundSampleEvent::Cow => {
                if time.elapsed() > sound_samples.last_moo_time + Duration::from_secs(5) {
                    audio.play(sound_samples.moo.clone());
                    sound_samples.last_moo_time = time.elapsed();
                }
            }
            SoundSampleEvent::EnginePower => {
                if let Some(engine) = audio_sinks.get(&sound_samples.engine_sound_sink) {
                    engine.set_volume(0.3);
                }
            }
            SoundSampleEvent::NoEnginePower => {
                if let Some(engine) = audio_sinks.get(&sound_samples.engine_sound_sink) {
                    engine.set_volume(0.2);
                }
            }
            SoundSampleEvent::HayBale => {
                if time.elapsed() > sound_samples.last_hay_time + Duration::from_millis(500) {
                    audio.play_with_settings(
                        sound_samples.hay.clone(),
                        PlaybackSettings {
                            volume: 1.0,
                            ..default()
                        },
                    );
                    sound_samples.last_hay_time = time.elapsed();
                }
            }
        }
    }
}
