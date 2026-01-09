#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationKeys {
    Gte,
    Lte,
    Gt,
    Lt,
    Eq,
    Neq,
}

impl OperationKeys {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Gte => "gte",
            Self::Lte => "lte",
            Self::Gt => "gt",
            Self::Lt => "lt",
            Self::Eq => "eq",
            Self::Neq => "neq",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gte" => Some(Self::Gte),
            "lte" => Some(Self::Lte),
            "gt" => Some(Self::Gt),
            "lt" => Some(Self::Lt),
            "eq" => Some(Self::Eq),
            "neq" => Some(Self::Neq),
            _ => None,
        }
    }

    pub const fn iter() -> [Self; 6] {
        [
            Self::Gte,
            Self::Lte,
            Self::Gt,
            Self::Lt,
            Self::Eq,
            Self::Neq,
        ]
    }

    pub fn contains(s: &str) -> bool {
        Self::from_str(s).is_some()
    }
}

#[cfg(test)]
mod fn_from_str_tests {
    use super::*;

    #[test]
    fn valid_str() {
        assert!(OperationKeys::from_str("neq").is_some())
    }

    #[test]
    fn invalid_str() {
        assert!(OperationKeys::from_str("abc").is_none())
    }
}
