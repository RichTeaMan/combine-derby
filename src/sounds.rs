use std::time::Duration;

use bevy::{audio::AudioSink, prelude::*};

use crate::{events::SoundSampleEvent, input::Settings};

const ENGINE_IDLE_VOLUME: f32 = 0.2;
const ENGINE_NO_IDLE_VOULME: f32 = 0.3;
const BACKGROUND_SOUND_VOLUME: f32 = 0.2;

#[derive(Component)]
pub struct SoundCollider {
    pub sound_sample: SoundSampleEvent,
}

pub struct SoundSample {
    audio_source: Handle<AudioSource>,
    last_play_time: Duration,
    /**
     * The time between samples should be started. Use this to stop several samples being played over each other.
     */
    interval: Duration,
    volume: f32,
}

impl SoundSample {
    pub fn new(audio_source: Handle<AudioSource>, interval: Duration, volume: f32) -> Self {
        SoundSample {
            audio_source,
            interval,
            last_play_time: Duration::ZERO,
            volume,
        }
    }

    pub fn play(&mut self, time: &Res<Time>, audio: &Res<Audio>, volume_modifier: f32) {
        if time.elapsed() > self.last_play_time + self.interval {
            audio.play_with_settings(
                self.audio_source.clone(),
                PlaybackSettings {
                    volume: self.volume * volume_modifier,
                    ..default()
                },
            );
            self.last_play_time = time.elapsed();
        }
    }
}

#[derive(Component)]
pub struct SoundSamples {
    pub moo: SoundSample,

    pub hay: SoundSample,

    pub engine_sound_sink: Handle<AudioSink>,

    pub background_sound_sink: Handle<AudioSink>,
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
            volume: ENGINE_IDLE_VOLUME,
            ..Default::default()
        },
    );

    let background_sound_sink = audio.play_with_settings(
        background,
        PlaybackSettings {
            repeat: true,
            volume: BACKGROUND_SOUND_VOLUME,
            ..Default::default()
        },
    );

    commands.spawn(SoundSamples {
        moo: SoundSample::new(moo, Duration::from_secs(5), 1.0),
        hay: SoundSample::new(hay, Duration::from_millis(500), 1.0),
        engine_sound_sink: audio_sinks.get_handle(engine_sound_sink),
        background_sound_sink: audio_sinks.get_handle(background_sound_sink),
    });
}

pub fn play_sample(
    mut sound_sample_events: EventReader<SoundSampleEvent>,
    mut sound_samples_query: Query<&mut SoundSamples>,
    audio: Res<Audio>,
    time: Res<Time>,
    audio_sinks: Res<Assets<AudioSink>>,
    settings: Res<Settings>,
) {
    let mut sound_samples = sound_samples_query.single_mut();

    for sound_sample_event in sound_sample_events.iter() {
        match sound_sample_event {
            SoundSampleEvent::Cow => {
                sound_samples.moo.play(&time, &audio, settings.volume);
            }
            SoundSampleEvent::EnginePower => {
                if let Some(engine) = audio_sinks.get(&sound_samples.engine_sound_sink) {
                    engine.set_volume(ENGINE_NO_IDLE_VOULME * settings.volume);
                }
            }
            SoundSampleEvent::NoEnginePower => {
                if let Some(engine) = audio_sinks.get(&sound_samples.engine_sound_sink) {
                    engine.set_volume(ENGINE_IDLE_VOLUME * settings.volume);
                }
            }
            SoundSampleEvent::HayBale => {
                sound_samples.hay.play(&time, &audio, settings.volume);
            }
        }
    }

    if let Some(background_sound) = audio_sinks.get(&sound_samples.background_sound_sink) {
        background_sound.set_volume(BACKGROUND_SOUND_VOLUME * settings.volume);
    }
}
