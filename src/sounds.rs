use std::time::Duration;

use bevy::prelude::*;

use crate::events::SoundSampleEvent;

#[derive(Component)]
pub struct SoundSamples {
    pub moo: Handle<AudioSource>,

    pub last_moo_time: Duration,
}

pub fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let moo: Handle<AudioSource> = asset_server.load("sounds/moo.ogg");

    commands.spawn().insert(SoundSamples { moo, last_moo_time: Duration::ZERO });
}

pub fn play_sample(
    mut sound_sample_events: EventReader<SoundSampleEvent>,
    mut sound_samples_query: Query<&mut SoundSamples>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    let mut sound_samples = sound_samples_query.single_mut();

    for sound_sample_event in sound_sample_events.iter() {
        match sound_sample_event {
            SoundSampleEvent::Cow => {
                if time.time_since_startup() > sound_samples.last_moo_time + Duration::from_secs(5)
                {
                    audio.play(sound_samples.moo.clone());
                    sound_samples.last_moo_time = time.time_since_startup();
                }
            }
        }
    }
}
