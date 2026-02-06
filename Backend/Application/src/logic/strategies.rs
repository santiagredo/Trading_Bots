use chrono::{Duration, Local, NaiveDateTime};

use models::structs::StrategyRequest;

use crate::utils::Utils;

/* ======================================================
 * VALIDATIONS
 * ======================================================
 */

pub fn validate_insert(req: &StrategyRequest) -> Result<(), String> {
    Utils::validate_empty_field(req.name.clone().unwrap_or_default(), "Strategy name")?;

    if req.cooldown.is_none_or(|v| v <= 0) {
        return Err(format!("Invalid cooldown: {:?}", req.cooldown));
    }

    if req.error_cooldown.is_none_or(|v| v <= 0) {
        return Err(format!("Invalid error cooldown: {:?}", req.error_cooldown));
    }

    Ok(())
}

pub fn validate_update(req: &StrategyRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid strategy id: {:?}", req.id));
    }

    if let Some(name) = req.name.clone() {
        Utils::validate_empty_field(name, "Strategy name")?;
    }

    if req.cooldown.is_some_and(|v| v <= 0) {
        return Err(format!("Invalid cooldown: {:?}", req.cooldown));
    }

    if req.error_cooldown.is_some_and(|v| v <= 0) {
        return Err(format!("Invalid error cooldown: {:?}", req.error_cooldown));
    }

    Ok(())
}

pub fn validate_delete(req: &StrategyRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err("Invalid strategy ID".into());
    }

    Ok(())
}

/* ======================================================
 * EVALUATION
 * ======================================================
 */

pub fn evaluate_cooldown(last_exec: Option<NaiveDateTime>, cooldown: Option<i32>) -> bool {
    if let Some(last_exec) = last_exec {
        let now = Local::now().naive_local();
        let cooldown_secs = cooldown.unwrap_or(86_400) as i64;

        now > last_exec + Duration::seconds(cooldown_secs)
    } else {
        true
    }
}

/* ======================================================
 * TESTS
 * ======================================================
 */

#[cfg(test)]
mod fn_validate_insert {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                StrategyRequest {
                    name: Some("Scalping".into()),
                    cooldown: Some(10),
                    error_cooldown: Some(10),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_empty_name",
                StrategyRequest {
                    name: Some("".into()),
                    cooldown: Some(10),
                    error_cooldown: Some(10),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_cooldown",
                StrategyRequest {
                    name: Some("A".into()),
                    cooldown: Some(0),
                    error_cooldown: Some(10),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_insert(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_update {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                StrategyRequest {
                    id: Some(1),
                    name: Some("Swing".into()),
                    cooldown: Some(30),
                    error_cooldown: Some(30),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id",
                StrategyRequest {
                    id: Some(0),
                    name: Some("Swing".into()),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_update(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_delete {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                StrategyRequest {
                    id: Some(1),
                    ..Default::default()
                },
                true,
            ),
            (
                "err",
                StrategyRequest {
                    id: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_delete(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_evaluate_cooldown {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn cases() {
        let now = Local::now().naive_local();

        let cases = vec![
            ("ok_none_last_exec", None, Some(60), true),
            (
                "ok_passed",
                Some(now - Duration::seconds(120)),
                Some(60),
                true,
            ),
            (
                "err_not_passed",
                Some(now - Duration::seconds(30)),
                Some(60),
                false,
            ),
            (
                "ok_default_cd",
                Some(now - Duration::seconds(90_000)),
                None,
                true,
            ),
            (
                "err_default_cd",
                Some(now - Duration::seconds(100)),
                None,
                false,
            ),
        ];

        for (name, last, cd, expected) in cases {
            let result = evaluate_cooldown(last, cd);
            assert_eq!(result, expected, "case `{}` failed", name);
        }
    }
}
