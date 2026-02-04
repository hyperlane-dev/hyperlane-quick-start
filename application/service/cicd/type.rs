use super::*;

pub(super) type StreamResult = Result<Result<String, String>, JoinError>;

pub(super) type StreamResultPair = (StreamResult, StreamResult);

pub(super) type TimeoutResult = Result<StreamResultPair, Elapsed>;
