use super::*;

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Next,
    Prev,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "next" => Ok(Direction::Next),
            "prev" => Ok(Direction::Prev),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}
