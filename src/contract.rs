#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TallyResponse};
use crate::state::{Config, Poll, VoteChoice, CONFIG, ID_COUNTER, POLLS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:heres-one-i-made-earlier";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Validate the address
    let admin_address = deps.api.addr_validate(&msg.admin_address)?;
    // Setup our config struct
    let config = Config {
        admin_address: admin_address.clone(), // Clone as it does not have the Copy trait
    };
    // Set the cw2 spec, using our contract name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Save our config and unwrap to assert success using ?
    CONFIG.save(deps.storage, &config)?;

    // Save the id counter
    ID_COUNTER.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin_address", admin_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Identify what message this is and call the correct function!
    match msg {
        ExecuteMsg::CreatePoll { question } => create_poll(deps, question),
        ExecuteMsg::Vote { poll_id, vote_type } => vote(deps, poll_id, vote_type),
    }
}

/// Create a poll!
fn create_poll(deps: DepsMut, question: String) -> Result<Response, ContractError> {
    // What is the current id? The key in our map
    // may_load can be used if you want to handle a case where
    // it does not exist.
    let current_id = ID_COUNTER.load(deps.storage)?;

    // Create the struct, the value in our map
    let poll = Poll {
        question,
        yes_votes: 0,
        no_votes: 0,
    };
    // Save to our state
    POLLS.save(deps.storage, current_id, &poll)?;

    // We need to increment the ID to be used after us
    // Example 1: new variable
    // let new_counter = current_id + 1;
    // ID_COUNTER.save(deps.storage, &new_counter)?;

    // Example 2: update
    ID_COUNTER.update(deps.storage, |v| -> StdResult<u64> {
        // Could also be
        // Ok(v + 1)
        // Used checked_add for a nice panic if we overflow
        Ok(v.checked_add(1).unwrap())
    })?;

    Ok(Response::new()
        .add_attribute("action", "create_poll")
        .add_attribute("poll_id", current_id.to_string()))
}

fn vote(deps: DepsMut, poll_id: u64, vote_type: VoteChoice) -> Result<Response, ContractError> {
    POLLS.update(
        deps.storage,
        poll_id,
        |poll| -> Result<Poll, ContractError> {
            if let Some(p) = poll {
                match vote_type {
                    VoteChoice::Yes => Ok(Poll {
                        yes_votes: p.yes_votes.checked_add(1).unwrap(),
                        ..p
                    }),
                    VoteChoice::No => Ok(Poll {
                        no_votes: p.no_votes.checked_add(1).unwrap(),
                        ..p
                    }),
                }
            } else {
                Err(ContractError::CustomError {
                    val: "Poll does not exist".to_string(),
                })
            }
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("poll_id", poll_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTally { poll_id } => to_binary(&get_tally(deps, poll_id)?),
    }
}

fn get_tally(deps: Deps, poll_id: u64) -> StdResult<TallyResponse> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;
    match poll {
        Some(p) => Ok(TallyResponse {
            yes_votes: p.yes_votes,
            no_votes: p.no_votes,
        }),
        None => Err(StdError::generic_err("Poll does not exist")),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
