#![allow(unused_variables)]
#![allow(unreachable_code)]

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{UserDetails, USERDETAILS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:user-attributes";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let user = UserDetails {
        first_name: msg.first_name,
        last_name: msg.last_name,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    USERDETAILS.save(deps.storage, &user)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UserDetails {
            first_name,
            last_name,
        } => execute_user(deps, env, info, first_name, last_name),
    }
}

pub fn execute_user(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    first_name: String,
    last_name: String,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("Vikas", first_name)
        .add_attribute("Chauhan", last_name))

    // Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::userdetails {} => to_binary(&query_user(_deps)?),
    }
}

pub fn query_user(deps: Deps) -> StdResult<UserDetails> {
    let user_info = USERDETAILS.load(deps.storage)?;
    Ok(UserDetails {
        first_name: user_info.first_name,
        last_name: user_info.last_name,
    })
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn instantiate(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> Result<Response, ContractError> {
//     unimplemented!()
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     unimplemented!()
// }

//#[cfg_attr(not(feature = "library"), entry_point)]
// pub fn user_details(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: UserDetails,
// ) -> Result<Response, ContractError> {
//     let res = Response::new()
//         .add_attribute("Vikas", msg.first_name)
//         .add_attribute("Chauhan", msg.last_name);
//     Ok(res)
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {}
