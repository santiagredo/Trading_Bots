use std::fmt::Debug;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub trait FiniteStateMachine: Copy + Sized + Debug {
    fn can_transition(self, next: Self) -> bool;
}

pub fn apply_fsm_transition<T: FiniteStateMachine>(current: &mut T, next: T) -> Result<(), String> {
    if !current.can_transition(next) {
        return Err(format!(
            "Invalid state transition: {:?} -> {:?}",
            current, next
        ));
    }

    *current = next;

    Ok(())
}

pub trait TimestampedState {
    type State: FiniteStateMachine;

    fn state_mut(&mut self) -> &mut Self::State;
    fn last_update_mut(&mut self) -> &mut NaiveDateTime;
}

pub fn transition_with_timestamp<T>(target: &mut T, next: T::State) -> Result<(), String>
where
    T: TimestampedState,
{
    apply_fsm_transition(target.state_mut(), next)?;
    *target.last_update_mut() = Local::now().naive_local();
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Off,
    Starting,
    Running,
    Stopping,
}

impl Default for LifecycleState {
    fn default() -> Self {
        LifecycleState::Off
    }
}

impl FiniteStateMachine for LifecycleState {
    fn can_transition(self, next: Self) -> bool {
        use LifecycleState::*;

        matches!(
            (self, next),
            (Off, Starting) | (Starting, Running) | (Running, Stopping) | (Stopping, Off)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadState {
    NotLoaded,
    Loaded,
    Degraded,
}

impl Default for LoadState {
    fn default() -> Self {
        LoadState::NotLoaded
    }
}

impl FiniteStateMachine for LoadState {
    fn can_transition(self, next: Self) -> bool {
        use LoadState::*;

        matches!(
            (self, next),
            (NotLoaded, Loaded) | (Loaded, Degraded) | (Degraded, Loaded) | (Degraded, NotLoaded)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingMode {
    Ready,
    Running,
    Trading,
    Saving,
}

impl Default for TradingMode {
    fn default() -> Self {
        TradingMode::Ready
    }
}

impl FiniteStateMachine for TradingMode {
    fn can_transition(self, next: Self) -> bool {
        use TradingMode::*;

        matches!(
            (self, next),
            (Ready, Running)
                | (Running, Trading)
                | (Trading, Saving)
                | (Saving, Running)
                | (Running, Ready)
                | (Saving, Ready)
        )
    }
}
