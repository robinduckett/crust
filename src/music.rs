use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};
use std::time::Duration;

type MusicValue = f32;

const MUSIC_TIMER_MS: u32 = 100;
const MUSIC_TIMER: MusicValue = MUSIC_TIMER_MS as MusicValue / 1000.0;

#[derive(PartialEq, Eq, Reflect)]
pub enum MusicStatus {
    Playing,
    Paused,
}

#[derive(Reflect)]
pub enum MusicStatusEvent {
    None,
    Play,
    Pause,
}

#[derive(Reflect)]
pub enum MusicTrackEvent {
    None,
    Begin,
    Interrupt,
    Fade,
}

#[derive(Reflect, Default)]
pub struct MusicTrack {
    name: String,
    volume: MusicValue,
    mood: MusicValue,
    threat: MusicValue,
    is_finished: bool,
}

impl MusicTrack {
    pub fn update(&mut self, volume: MusicValue, _time: MusicValue) {
        self.volume = volume;
        // self.time = time;
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn begin_playing(&mut self, _volume: MusicValue, _time: MusicValue) {
        self.is_finished = false;
    }
}

#[derive(Resource, Reflect)]
pub struct MusicManager {
    next_audio_update: u32,
    status: MusicStatus,
    status_event: MusicStatusEvent,
    track_event: MusicTrackEvent,
    next_track_name: String,
    volume: MusicValue,
    current_mood: MusicValue,
    target_mood: MusicValue,
    current_threat: MusicValue,
    target_threat: MusicValue,
    tracks: HashMap<String, MusicTrack>,
    current_track: Option<MusicTrack>,
    old_track: Option<MusicTrack>,
    time: MusicValue,
    current_reading: u32,
}

impl MusicManager {
    pub fn new(time: u32) -> Self {
        Self {
            next_audio_update: time,
            status: MusicStatus::Paused,
            status_event: MusicStatusEvent::None,
            track_event: MusicTrackEvent::None,
            next_track_name: String::new(),
            volume: 0.0,
            current_mood: 0.0,
            target_mood: 0.0,
            current_threat: 0.0,
            target_threat: 0.0,
            tracks: HashMap::new(),
            current_track: None,
            old_track: None,
            time: 0.0,
            current_reading: 0,
        }
    }

    pub fn update(&mut self, time: u32) {
        let current_time = time;

        if current_time >= self.next_audio_update {
            self.next_audio_update = current_time + MUSIC_TIMER_MS;

            match self.status_event {
                MusicStatusEvent::None => {}
                MusicStatusEvent::Play => {
                    self.activate_play();
                }
                MusicStatusEvent::Pause => {
                    self.activate_pause();
                }
            }

            match self.track_event {
                MusicTrackEvent::None => {}
                MusicTrackEvent::Begin => {
                    self.activate_begin(self.next_track_name.clone());
                }
                MusicTrackEvent::Interrupt => {
                    self.activate_interrupt(self.next_track_name.clone());
                }
                MusicTrackEvent::Fade => {
                    self.activate_fade();
                }
            }

            self.status_event = MusicStatusEvent::None;
            self.track_event = MusicTrackEvent::None;

            if self.status != MusicStatus::Playing {
                return;
            }

            self.time += MUSIC_TIMER;

            // update music

            self.approach_target_mood(0.05);
            self.approach_target_threat(0.1);

            if self.old_track.is_some() {
                let old_track = self.old_track.as_mut().unwrap();
                old_track.update(self.volume, self.time);

                if old_track.is_finished() {
                    self.old_track = None;

                    if self.current_track.is_some() {
                        let current_track = self.current_track.as_mut().unwrap();
                        current_track.begin_playing(self.volume, self.time);
                    }
                }
            } else if self.current_track.is_some() {
                let current_track = self.current_track.as_mut().unwrap();
                current_track.update(self.volume, self.time);
            }
        }
    }

    fn approach(&mut self, current: &mut MusicValue, target: &MusicValue, amount: MusicValue) {
        let difference = *target - *current;

        if difference <= amount && difference >= -amount {
            *current = *target
        } else if difference > 0.0 {
            *current += amount
        } else {
            *current -= amount
        }
    }

    fn approach_target_mood(&mut self, amount: MusicValue) {
        let mut mood = self.current_mood;
        let target = self.target_mood;
        self.approach(&mut mood, &target, amount);
        self.current_mood = mood;
    }

    fn approach_target_threat(&mut self, amount: MusicValue) {
        let mut threat = self.current_threat;
        let target = self.target_threat;
        self.approach(&mut threat, &target, amount);
        self.current_threat = threat;
    }

    fn activate_play(&mut self) {
        self.status = MusicStatus::Playing;
    }

    fn activate_pause(&mut self) {
        self.status = MusicStatus::Paused;
    }

    fn activate_begin(&mut self, track_name: String) {}

    fn activate_interrupt(&mut self, track_name: String) {}

    fn activate_fade(&mut self) {}
}

#[derive(Event, Default)]
pub struct StatusPlayEvent;

#[derive(Event, Default)]
pub struct StatusPauseEvent;

#[derive(Event, Default)]
pub struct TrackBeginEvent;

#[derive(Event, Default)]
pub struct TrackInterruptEvent;

#[derive(Event, Default)]
pub struct TrackFadeEvent;

pub struct GameMusicPlugin;

impl Plugin for GameMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            update_music.run_if(on_timer(Duration::from_millis(100))),
        );
        app.add_event::<StatusPlayEvent>();
        app.add_event::<StatusPauseEvent>();
        app.add_event::<TrackBeginEvent>();
        app.add_event::<TrackInterruptEvent>();
        app.add_event::<TrackFadeEvent>();
    }
}

fn setup(mut commands: Commands, time: Res<Time<Virtual>>) {
    commands.insert_resource(MusicManager::new(time.elapsed().as_millis() as u32));
}

fn update_music(mut music_manager: ResMut<MusicManager>, time: Res<Time<Virtual>>) {
    music_manager.update(time.elapsed().as_millis() as u32);
}
