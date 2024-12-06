#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::Map;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token_tracker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage for user token preferences
const USER_TOKENS: Map<&str, Vec<String>> = Map::new("user_tokens");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: (), // No specific instantiation message
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPreferences { tokens } => set_preferences(deps, info, tokens),
    }
}

pub fn set_preferences(
    deps: DepsMut,
    info: MessageInfo,
    tokens: Vec<String>,
) -> Result<Response, ContractError> {
    USER_TOKENS.save(deps.storage, info.sender.as_str(), &tokens)?;
    Ok(Response::new()
        .add_attribute("action", "set_preferences")
        .add_attribute("user", info.sender.to_string())
        .add_attribute("tokens", format!("{:?}", tokens)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPreferences { user } => to_json_binary(&get_preferences(deps, user)?),
    }
}

pub fn get_preferences(deps: Deps, user: String) -> StdResult<Vec<String>> {
    let tokens = USER_TOKENS.may_load(deps.storage, user.as_str())?;
    Ok(tokens.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{MessageInfo, Addr, Coin, from_json};

    fn mock_message_info(sender: &str, funds: &[Coin]) -> MessageInfo {
        MessageInfo {
            sender: Addr::unchecked(sender),
            funds: funds.to_vec(),
        }
    }

    #[test]
    fn set_and_get_preferences() {
        let mut deps = mock_dependencies();

        // User sets token preferences
        let info = mock_message_info("user1", &[]);
        let tokens = vec!["ATOM".to_string(), "OSMO".to_string()];
        let msg = ExecuteMsg::SetPreferences { tokens: tokens.clone() };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Query the preferences
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetPreferences {
                user: info.sender.to_string(),
            },
        )
        .unwrap();
        let value: Vec<String> = from_json(&res).unwrap();
        assert_eq!(tokens, value);
    }
}
