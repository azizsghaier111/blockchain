use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::msg::{CountResponse,DotProductResponse,VectorResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        admin_vector: vec![],
        x_vector: vec![],
};

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps, env),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::SetUserVector {value} => try_set_user_vector(deps, env, value),
        ExecuteMsg::SetAdminVector {admin_vector} => try_set_admin_vector(deps,info,admin_vector),
    }
}

pub fn try_increment(deps: DepsMut, _env: Env) -> StdResult<Response> {
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        state.count += 1;
        Ok(state)
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        state.count = count;
        Ok(state)
    })?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}
pub fn try_set_admin_vector(deps: DepsMut, info: MessageInfo, admin_vector: Vec<i32>) -> StdResult<Response> {
    let mut state = config(deps.storage).load()?;
    
    // Check if the message sender is the admin (owner)
    if info.sender != state.owner {
        return Err(StdError::generic_err("Only the contract owner can set the admin vector"));
    }

    // Update the admin vector in the contract state
    state.admin_vector = admin_vector;
    config(deps.storage).save(&state)?;

    deps.api.debug("Admin vector set successfully");
    Ok(Response::default())
}

pub fn try_set_user_vector(deps: DepsMut, _env: Env, value: i32) -> StdResult<Response> {
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        state.x_vector.push(value);
        Ok(state)
    })?;

    deps.api.debug("value added to array successfully");
    Ok(Response::default())
}



#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetAdminVector {} => to_binary(&query_admin_vector(deps)?),
        QueryMsg::GetDotProduct {} => to_binary(&query_dot_product(deps)?),
        QueryMsg::GetUserVector {} => to_binary(&query_user_vector(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let response = CountResponse { count: state.count };
    to_binary(&response)
}

fn query_admin_vector(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let response:   VectorResponse= VectorResponse { vector: state.admin_vector };
    to_binary(&response)
}
fn query_user_vector(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let response = VectorResponse { vector: state.x_vector };
    to_binary(&response)
}

fn query_dot_product(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    
    // Ensure both vectors are initialized and have the same length
    if state.admin_vector.is_empty() || state.x_vector.is_empty() ||
        state.admin_vector.len() != state.x_vector.len() {
        return Err(StdError::generic_err("Invalid vectors for dot product calculation"));
    }

    // Calculate the dot product
    let dot_product = calculate_dot_product(&state.admin_vector, &state.x_vector);

    let response = DotProductResponse { dot_product };
    to_binary(&response)
}

fn calculate_dot_product(vector_a: &[i32], vector_b: &[i32]) -> i32 {
    vector_a.iter().zip(vector_b.iter()).map(|(&a, &b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let init_msg = InstantiateMsg { count: 17 };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { count: 17 };

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // anyone can increment
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let exec_msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { count: 17 };

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // not anyone can reset
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let exec_msg = ExecuteMsg::Reset { count: 5 };

        let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

        match res {
            Err(StdError::GenericErr { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let exec_msg = ExecuteMsg::Reset { count: 5 };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}