use std::time::Duration;

use super::random_signal::RandomSignal;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        volume: u16,
        signal: RandomSignal,
        sparkline_data: Vec<u64>
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let sparkline_data =  signal.by_ref().take(200).collect::<Vec<u64>>();

        Self::Initialized {
            duration,
            counter_sleep,
            volume: counter_tick,
            signal,
            sparkline_data
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_volume(&mut self) {
        if let Self::Initialized { volume, .. } = self {
            *volume += 10;
            if volume >= &mut 100 {
                *volume = 100;
            }
        }
    }

    pub fn decr_volume(&mut self) {
        if let Self::Initialized { volume, .. } = self {
            if volume >= &mut 10 {
                *volume -= 10;
            }
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn volume(&self) -> Option<u16> {
        if let Self::Initialized { volume, .. } = self {
            Some(*volume)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
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

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
