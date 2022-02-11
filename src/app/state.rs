
use crate::playback;

use super::random_signal::RandomSignal;

pub enum AppState {
    Init,
    Initialized {
        volume: u16,
        signal: RandomSignal,
        sparkline_data: Vec<u64>,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let current_volume = 100;
        let sparkline_data =  signal.by_ref().take(200).collect::<Vec<u64>>();

        Self::Initialized {
            volume: current_volume,
            signal,
            sparkline_data,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_volume(&mut self) {
        if let Self::Initialized { volume, .. } = self {
            *volume += 10;
            if volume >= &mut 100 {
                *volume = 100;
            }
            playback::set_cmd(volume.to_owned());
        }
    }

    pub fn decr_volume(&mut self) {
        if let Self::Initialized { volume, .. } = self {
            let step: u16 = 10;
            if volume.clone() - step >= step {
                *volume -= 10;
            }
            playback::set_cmd(volume.to_owned());
        }
    }

    pub fn volume(&self) -> Option<u16> {
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
