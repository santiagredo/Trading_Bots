use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::enums::MetricType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metric {
    pub metric_type: MetricType,
    pub latest_update: DateTime,
    pub total_duration: Duration,
    pub count: u128,
    pub fastest_duration: DurationDate,
    pub slowest_duration: DurationDate,
}

impl Metric {
    pub fn default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add(&mut self, elapsed: Duration, date_time: DateTime) {
        self.total_duration += elapsed;
        self.count += 1;

        self.fastest_duration.check(elapsed, date_time, true);
        self.slowest_duration.check(elapsed, date_time, false);
        self.latest_update = date_time;
    }

    pub fn average(&self) -> Duration {
        if self.count == 0 {
            return Duration::ZERO;
        }

        let total_nanos = self.total_duration.as_nanos();

        let avg_nanos = total_nanos / self.count as u128;

        Duration::from_nanos(avg_nanos as u64)
    }

    pub fn reset(&mut self) {
        self.total_duration = Duration::ZERO;
        self.count = 0;
        self.fastest_duration = DurationDate::default();
        self.slowest_duration = DurationDate::default();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DurationDate {
    pub duration: Duration,
    pub date_time: DateTime,
}

impl DurationDate {
    pub fn default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn check(&mut self, elapsed: Duration, date_time: DateTime, is_fastest: bool) {
        let should_update = if is_fastest {
            elapsed < self.duration || self.duration == Duration::ZERO
        } else {
            elapsed > self.duration
        };

        if should_update {
            self.duration = elapsed;
            self.date_time = date_time;
        }
    }
}
