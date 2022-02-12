
use std::sync::mpsc::Sender;

use log::error;

use crate::playback::{self, PlaybackControl};

use super::random_signal::RandomSignal;

macro_rules! log_error {
    ($fmt:expr) => {{
        if let Err(err) = $fmt {
            error!("{}", err.to_string());
        }
    }};
}

pub enum AppState {
    Init,
    Initialized {
        volume: f32,
        signal: RandomSignal,
        sparkline_data: Vec<u64>,
        playback_remote: Sender<PlaybackControl>
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let current_volume = 1.00;
        let sparkline_data =  signal.by_ref().take(200).collect::<Vec<u64>>();

        let playback_remote = playback::start_playback();
        log_error!(playback_remote.send(PlaybackControl::VolumeUp(2.0)));

        Self::Initialized {
            volume: current_volume,
            signal,
            sparkline_data,
            playback_remote
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_volume(&mut self) {
        if let Self::Initialized { volume, playback_remote, .. } = self {
            let volume_step = 0.1;
            if volume.clone() + volume_step >= 1.00 {
                *volume = 1.0;
            }
            else {
                *volume = volume.clone() + volume_step;
            }

            log_error!(playback_remote.send(PlaybackControl::VolumeUp(*volume * 200.00 / 100.00)));
        }
    }

    pub fn decr_volume(&mut self) {
        if let Self::Initialized { volume, playback_remote, .. } = self {
            let step = 0.1;
            if volume.clone() - step >= step {
                *volume -= step;
            }
            else {
                *volume = 0.0;
            }
            log_error!(playback_remote.send(PlaybackControl::VolumeDown(*volume * 200.00 / 100.00)));
        }
    }

    pub fn volume(&self) -> Option<f32> {
        if let Self::Initialized { volume, .. } = self {
            Some(*volume)
        } else {
            None
        }
    }

    pub fn sparkline_data(&self) -> Option<&Vec<u64>> {
        if let Self::Initialized { sparkline_data, .. } = self {
            Some(sparkline_data)
        }
        else {
            None
        }
    }

    pub fn update_sparkline(&mut self) {
        if let Self::Initialized { sparkline_data, signal, .. } = self {
            let value = signal.next().unwrap();
            sparkline_data.pop();
            sparkline_data.insert(0, value);
        }
    }

}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
