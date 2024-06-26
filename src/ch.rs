use cosmwasm_std::Addr;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use crate::msg::{CountResponse,DotProductResponse,VectorResponse, ExecuteMsg, InstantiateMsg, QueryMsg,StrResponse};
use crate::state::{config, config_read, State};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,  // It provides functionalities to interact with the contract's storage,
    _env: Env, //The Env variable contains information about the current execution environment, including details such as the block time, block height, chain ID
    info: MessageInfo,   //his variable contains information about the message that triggered the execution of the contract function. It includes details such as the sender's address (info.sender
    msg: InstantiateMsg,  //This variable represents the instantiation message used when the contract is deployed for the first time. It contains specific information required to initialize the contract state, such as initial configuration parameters or values.
) -> StdResult<Response> {
    let state = State {
        owner: info.sender.clone(),
        admin_vector: vec![],
        x_vector: vec![],
        legit: vec![(info.sender.clone()).to_string()],
};
    println!("owner address: {:p}", &state.owner as *const Addr);
    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
        let state = config(deps.storage).load()?; // Load the state from storage

    // Check if the sender is not in the 'legit' vector in the state
    if !state.legit.contains(&(info.sender).to_string()) {
        return Err(StdError::generic_err("Only the contract owner can set the admin vector"));
    }
    match msg {
        ExecuteMsg::ResetX {} => try_reset_X(deps, info),
        ExecuteMsg::ResetAdmin {} => try_reset_A(deps, info),
        ExecuteMsg::ResetLegit {} => try_reset_legit(deps, info),
        ExecuteMsg::SetUserVector {value} => try_set_user_vector(deps, env, value),
        ExecuteMsg::SetAdminVector {admin_vector} => try_set_admin_vector(deps,info,admin_vector),
        ExecuteMsg::SetLegitimUsers {address}  => try_set_legitim_users(deps,info,address),
    }
}



pub fn try_reset_X(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        state.x_vector = Vec::new(); 
        Ok(state)
    })?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}
pub fn try_reset_A(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        state.admin_vector = Vec::new();
        Ok(state)
    })?;

    deps.api.debug("X reset successfully");
    Ok(Response::default())
}
pub fn try_reset_legit(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        state.legit = Vec::new();
        Ok(state)
    })?;

    deps.api.debug("A reset successfully");
    Ok(Response::default())
}


pub fn try_set_legitim_users(deps: DepsMut, info: MessageInfo, address: String) -> StdResult<Response> {
    let state = config(deps.storage).load()?;
    
    // Check if the message sender is the admin (owner)
    if info.sender != state.owner {
        return Err(StdError::generic_err("Only the contract owner can set the admin vector"));
    }

    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        state.legit.push(address);
        Ok(state)
    })?;

    deps.api.debug("Legit reset successfully");
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
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdminVector {} => to_binary(&query_admin_vector(deps)?),
        QueryMsg::GetDotProduct {} => to_binary(&query_dot_product(deps)?),
        QueryMsg::GetUserVector {} => to_binary(&query_user_vector(deps)?),
        QueryMsg::GetLegitVector {} => to_binary(&query_legitim_user(deps)?),
    }
}




// Use serde_json or another format to serialize your struct into a Vec<u8>
fn query_legitim_user(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
   // if info.sender != state.owner {
       // return Err(StdError::generic_err("Only the contract owner can get the admin vector"));
   // }
    let response = StrResponse { vector: state.legit };
    // Serialize the response into JSON and convert it into a Binary type
    let json = serde_json::to_vec(&response).map_err(|e| StdError::generic_err(format!("JSON serialization error: {}", e)))?;

    Ok(Binary(json))
}
fn query_admin_vector(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    //if info.sender != state.owner  {
      //  return Err(StdError::generic_err("Only the contract owner cans get the admin vector"));
    //}
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
   // if info.sender != state.owner {
       // return Err(StdError::generic_err("Only the contract owner can get the admin vector"));
   // }
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