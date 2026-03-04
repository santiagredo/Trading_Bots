use std::fmt::{self, Debug, Display};

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub trait FiniteStateMachine: Copy + Sized + Debug + PartialEq {
    fn can_transition(self, next: Self) -> bool;

    fn allows(self, required: Self) -> bool {
        self == required
    }
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

impl Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LifecycleState::Off => "off",
            LifecycleState::Starting => "starting",
            LifecycleState::Running => "running",
            LifecycleState::Stopping => "stopping",
        };

        write!(f, "{s}")
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
pub enum TradingState {
    Ready,
    Running,
    Trading,
    Saving,
}

impl Default for TradingState {
    fn default() -> Self {
        TradingState::Ready
    }
}

impl FiniteStateMachine for TradingState {
    fn can_transition(self, next: Self) -> bool {
        use TradingState::*;

        matches!(
            (self, next),
            (Ready, Running)
                | (Running, Trading)
                | (Trading, Saving)
                | (Trading, Ready)
                | (Running, Ready)
                | (Saving, Ready)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Sleeping,
    Running,
    Saving,
    Stopped,
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState::Sleeping
    }
}

// impl FiniteStateMachine for TaskState {
//     fn can_transition(self, next: Self) -> bool {
//         use TaskState::*;

//         matches!(
//             (self, next),
//             (Sleeping, Running) | (Running, Saving) | (Saving, Sleeping)
//         )
//     }
// }
