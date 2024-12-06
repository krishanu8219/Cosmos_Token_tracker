use crate::msg::QueryMsg;
use cosmwasm_std::{to_json_binary, StdResult, Binary};

pub fn build_get_preferences_query(user: &str) -> StdResult<Binary> {
    let msg = QueryMsg::GetPreferences {
        user: user.to_string(),
    };
    to_json_binary(&msg)
}
