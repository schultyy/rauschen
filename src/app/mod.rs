use crate::{inputs::key::Key};

use self::{
    actions::{Action, Actions},
    state::AppState,
};

mod actions;
mod random_signal;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
    /// Contextual actions
    actions: Actions,
}

impl App {
    pub fn new() -> Self {
        let actions = vec![Action::Quit, Action::VolumeUp, Action::VolumeDown].into();
        let state = AppState::initialized();

        Self { actions, state }
    }

    /// Handle a user action
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
                Action::VolumeUp => {
                    self.state.incr_volume();
                    AppReturn::Continue
                },
                Action::VolumeDown => {
                    self.state.decr_volume();
                    AppReturn::Continue
                }
            }
        } else {
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.update_sparkline();
        AppReturn::Continue
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}
