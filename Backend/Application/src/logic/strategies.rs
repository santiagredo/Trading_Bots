use chrono::{Duration, Local, NaiveDateTime};

use crate::{
    handler::Strategies,
    utils::{Logic, Utils},
};

impl Strategies<Logic> {
    pub fn insert_strategy_logic(self) -> Result<Self, String> {
        Utils::validate_empty_field(self.model.name.clone().unwrap_or_default(), "Strategy name")?;

        if self.model.cooldown.is_none_or(|cooldown| cooldown == 0) {
            return Err(format!("Invalid cooldown: {:?}", self.model.cooldown));
        };

        Ok(self)
    }

    pub fn update_strategy_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid strategy id: {:?}", self.model.id));
        }

        if let Some(name) = self.model.name.clone() {
            Utils::validate_empty_field(name, "Strategy name")?;
        }

        if self.model.cooldown.is_some_and(|cooldown| cooldown <= 0) {
            return Err(format!("Invalid cooldown: {:?}", self.model.cooldown));
        };

        Ok(self)
    }

    pub fn delete_strategy_logic(self) -> Result<Self, String> {
        if self.model.id.is_none() || self.model.id.is_some_and(|id| id <= 0) {
            return Err(format!("Invalid strategy ID"));
        }

        Ok(self)
    }

    pub fn evaluate_cooldown_logic(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        if let Some(last_exec) = last_exec {
            let now = Local::now().naive_local();

            let cooldown_secs = cooldown.unwrap_or(86400) as i64;
            let cooldown_end = last_exec + Duration::seconds(cooldown_secs);

            now > cooldown_end
        } else {
            true
        }
    }
}

#[cfg(test)]
mod fn_insert_strategy_logic {
    use crate::handler::Strategies;
    use crate::utils::Logic;

    #[test]
    fn insert_strategy_cases() {
        let cases = vec![
            (
                "ok_valid_insert",
                Some("Scalping".to_string()),
                Some(10),
                true,
            ),
            ("err_empty_name", Some("".to_string()), Some(10), false),
            ("err_none_name", None, Some(10), false),
            (
                "err_invalid_cooldown",
                Some("Scalping".to_string()),
                Some(0),
                false,
            ),
            (
                "err_none_cooldown",
                Some("Scalping".to_string()),
                None,
                false,
            ),
        ];

        for (name, strategy_name, cooldown, should_pass) in cases {
            let mut strategy = Strategies::default();
            strategy.model.name = strategy_name;
            strategy.model.cooldown = cooldown;

            let result = strategy.next_phase::<Logic>().insert_strategy_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_update_strategy_logic {
    use crate::handler::Strategies;
    use crate::utils::Logic;

    #[test]
    fn update_strategy_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                Some(1),
                Some("Swing".to_string()),
                Some(30),
                true,
            ),
            (
                "err_invalid_id",
                Some(0),
                Some("Swing".to_string()),
                Some(30),
                false,
            ),
            (
                "err_none_id",
                None,
                Some("Swing".to_string()),
                Some(30),
                false,
            ),
            (
                "err_empty_name",
                Some(1),
                Some("".to_string()),
                Some(30),
                false,
            ),
            (
                "err_invalid_cooldown",
                Some(1),
                Some("Swing".to_string()),
                Some(0),
                false,
            ),
        ];

        for (name, id, strategy_name, cooldown, should_pass) in cases {
            let mut strategy = Strategies::default();
            strategy.model.id = id;
            strategy.model.name = strategy_name;
            strategy.model.cooldown = cooldown;

            let result = strategy.next_phase::<Logic>().update_strategy_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_delete_strategy_logic {
    use crate::handler::Strategies;
    use crate::utils::Logic;

    #[test]
    fn delete_strategy_cases() {
        let cases = vec![
            ("ok_valid_delete", Some(1), true),
            ("err_none_id", None, false),
            ("err_invalid_id", Some(0), false),
        ];

        for (name, id, should_pass) in cases {
            let mut strategy = Strategies::default();
            strategy.model.id = id;

            let result = strategy.next_phase::<Logic>().delete_strategy_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_evaluate_cooldown_logic {
    use crate::handler::Strategies;
    use crate::utils::Logic;
    use chrono::{Duration, Local};

    #[test]
    fn evaluate_cooldown_cases() {
        let now = Local::now().naive_local();

        let cases = vec![
            ("ok_none_last_exec", None, Some(60), true),
            (
                "ok_passed_cooldown",
                Some(now - Duration::seconds(120)),
                Some(60),
                true,
            ),
            (
                "err_not_passed_cooldown",
                Some(now - Duration::seconds(30)),
                Some(60),
                false,
            ),
            (
                "ok_none_cooldown_default",
                Some(now - Duration::seconds(86500)),
                None,
                true,
            ),
            (
                "err_none_cooldown_still_active",
                Some(now - Duration::seconds(100)),
                None,
                false,
            ),
        ];

        for (name, last_exec, cooldown, expected) in cases {
            let strategy = Strategies::default();

            let result = strategy
                .next_phase::<Logic>()
                .evaluate_cooldown_logic(last_exec, cooldown);

            assert_eq!(
                result, expected,
                "case `{}` failed: expected {}, got {}",
                name, expected, result
            );
        }
    }
}
