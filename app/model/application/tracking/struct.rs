use super::*;

#[derive(Data, DisplayDebug, CustomDebug, Deserialize, Default)]
pub struct Tracking {
    url: String,
}
