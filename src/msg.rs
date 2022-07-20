use crate::state::VoteChoice;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Contract admin - can delete any user's polls
    /// this is a string because we will
    /// validate & save later
    pub admin_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// In our spec a poll is made up of a question
    /// and a yes/no vote tally.
    CreatePoll { question: String },
    /// Identify a poll via a poll_id (u64)
    /// and vote yes or no
    Vote { poll_id: u64, vote_type: VoteChoice },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get an inflight poll
    GetTally { poll_id: u64 },
}

// We define a custom struct for each query response
// this allows us to deserialize our responses nicely
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TallyResponse {
    pub yes_votes: u64,
    pub no_votes: u64,
}
