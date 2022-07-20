use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Validated in the instantiate method,
    /// so we can now store it as an Addr.
    pub admin_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum VoteChoice {
    /// In JSON this will be "yes"
    Yes,
    /// In JSON this will be "no"
    No,
}

// Create the Item of type Config under the storage key "config"
pub const CONFIG: Item<Config> = Item::new("config");
// Create the Item of type u64 under the storage key "id_counter"
pub const ID_COUNTER: Item<u64> = Item::new("id_counter");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

// Create a mapping of id to Poll
// Key will be u64
// Value will be a poll
// e.g. 0 -> Poll { question: "Do you like cats?", yes_votes: 0, no_votes: 10000 }
pub const POLLS: Map<u64, Poll> = Map::new("polls");
