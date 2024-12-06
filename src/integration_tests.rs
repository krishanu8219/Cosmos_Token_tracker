#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::{Addr, Coin, MessageInfo};
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::contract::{execute, query};

    // Mock function to create MessageInfo
    fn mock_message_info(sender: &str, funds: &[Coin]) -> MessageInfo {
        MessageInfo {
            sender: Addr::unchecked(sender),
            funds: funds.to_vec(),
        }
    }

    #[test]
    fn test_set_preferences() {
        let mut deps = cosmwasm_std::testing::mock_dependencies();

        // Set token preferences
        let info = mock_message_info("user1", &[]);
        let tokens = vec!["ATOM".to_string(), "OSMO".to_string()];
        let msg = ExecuteMsg::SetPreferences { tokens: tokens.clone() };
        let _res = execute(deps.as_mut(), cosmwasm_std::testing::mock_env(), info.clone(), msg).unwrap();

        // Query preferences
        let query_msg = QueryMsg::GetPreferences {
            user: "user1".to_string(),
        };
        let res = query(deps.as_ref(), cosmwasm_std::testing::mock_env(), query_msg).unwrap();
        let preferences: Vec<String> = serde_json_wasm::from_slice(&res).unwrap();

        assert_eq!(preferences, tokens);
    }
}
