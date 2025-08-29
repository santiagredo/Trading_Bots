use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Lte,
    Lt,
    Gte,
    Gt,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lte" => Ok(Direction::Lte),
            "gte" => Ok(Direction::Gte),
            "lt" => Ok(Direction::Lt),
            "gt" => Ok(Direction::Gt),
            _ => Err(()),
        }
    }
}
