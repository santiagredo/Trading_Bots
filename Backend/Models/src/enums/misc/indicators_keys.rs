#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorKeys {
    Pch,
    Pcp,
    Wap,
    Lst,
    Opn,
    Hgh,
    Low,
    Bav,
    Qav,
    Ath,
    Pfath,
}

impl IndicatorKeys {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pch => "pch",
            Self::Pcp => "pcp",
            Self::Wap => "wap",
            Self::Lst => "lst",
            Self::Opn => "opn",
            Self::Hgh => "hgh",
            Self::Low => "low",
            Self::Bav => "bav",
            Self::Qav => "qav",
            Self::Ath => "ath",
            Self::Pfath => "pfath",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pch" => Some(Self::Pch),
            "pcp" => Some(Self::Pcp),
            "wap" => Some(Self::Wap),
            "lst" => Some(Self::Lst),
            "opn" => Some(Self::Opn),
            "hgh" => Some(Self::Hgh),
            "low" => Some(Self::Low),
            "bav" => Some(Self::Bav),
            "qav" => Some(Self::Qav),
            "ath" => Some(Self::Ath),
            "pfath" => Some(Self::Pfath),
            _ => None,
        }
    }

    pub const fn iter() -> [Self; 11] {
        [
            Self::Pch,
            Self::Pcp,
            Self::Wap,
            Self::Lst,
            Self::Opn,
            Self::Hgh,
            Self::Low,
            Self::Bav,
            Self::Qav,
            Self::Ath,
            Self::Pfath,
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
        assert!(IndicatorKeys::from_str("pfath").is_some())
    }

    #[test]
    fn invalid_str() {
        assert!(IndicatorKeys::from_str("abc").is_none())
    }
}
