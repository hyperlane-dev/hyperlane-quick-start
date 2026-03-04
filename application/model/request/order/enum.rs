use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Next,
    Prev,
}
