#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use cw2::set_contract_version;
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, query_balance, query_token_info,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::error::ContractError;
use crate::msg::{BlockedResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::BLOCKED;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-blocklist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // store token info using cw20-base format
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: Uint128::zero(),
        mint: Some(MinterData {
            minter: info.sender.clone(),
            cap: None,
        }),
    };
    TOKEN_INFO.save(deps.storage, &data)?;

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
        ExecuteMsg::AddToBlockedList { address } => Ok(try_add_to_blocklist(deps, info, address)?),
        ExecuteMsg::RemoveFromBlockedList { address } => {
            Ok(try_remove_from_blocklist(deps, info, address)?)
        }

        ExecuteMsg::Mint { recipient, amount } => {
            Ok(execute_mint(deps, env, info, recipient, amount)?)
        }

        // these all come from cw20-base to implement the cw20 standard
        ExecuteMsg::Transfer { recipient, amount } => {
            if is_blocked(deps.as_ref(), info.sender.to_string()).unwrap_or_default() {
                return Err(ContractError::Blocked {});
            }
            Ok(execute_transfer(deps, env, info, recipient, amount)?)
        }
        ExecuteMsg::Burn { amount } => {
            let config = TOKEN_INFO.load(deps.storage)?;
            if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            Ok(execute_burn(deps, env, info, amount)?)
        }
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => {
            if is_blocked(deps.as_ref(), info.sender.to_string()).unwrap_or_default() {
                return Err(ContractError::Blocked {});
            }
            Ok(execute_send(deps, env, info, contract, amount, msg)?)
        }
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => {
            if is_blocked(deps.as_ref(), owner.to_string()).unwrap_or_default() {
                return Err(ContractError::Blocked {});
            }
            Ok(execute_transfer_from(
                deps, env, info, owner, recipient, amount,
            )?)
        }
        ExecuteMsg::BurnFrom { owner, amount } => {
            let config = TOKEN_INFO.load(deps.storage)?;
            if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
                return Err(ContractError::Unauthorized {});
            }
            Ok(execute_burn_from(deps, env, info, owner, amount)?)
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => {
            if is_blocked(deps.as_ref(), owner.to_string()).unwrap_or_default() {
                return Err(ContractError::Blocked {});
            }
            Ok(execute_send_from(
                deps, env, info, owner, contract, amount, msg,
            )?)
        }
    }
}

pub fn try_add_to_blocklist(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let address_to_block = deps
        .api
        .addr_humanize(&deps.api.addr_canonicalize(&address).unwrap())
        .unwrap();

    BLOCKED.save(deps.storage, &address_to_block, &true)?;

    Ok(Response::new().add_attribute("blocked", "true"))
}

pub fn try_remove_from_blocklist(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let address_to_unblock = deps
        .api
        .addr_humanize(&deps.api.addr_canonicalize(&address).unwrap())
        .unwrap();

    BLOCKED.save(deps.storage, &address_to_unblock, &false)?;

    Ok(Response::new().add_attribute("blocked", "false"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsBlocked { address } => to_binary(&query_blocked(deps, address)?),
        // inherited from cw20-base
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
    }
}

fn is_blocked(deps: Deps, address: String) -> Option<bool> {
    let address_to_check = deps
        .api
        .addr_humanize(&deps.api.addr_canonicalize(&address).unwrap())
        .unwrap();

    return BLOCKED
        .may_load(deps.storage, &address_to_check)
        .unwrap_or_default();
}
fn query_blocked(deps: Deps, address: String) -> StdResult<BlockedResponse> {
    Ok(BlockedResponse {
        blocked: is_blocked(deps, address).unwrap_or_default(),
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw20::TokenInfoResponse;

    use super::*;
    fn get_balance<T: Into<String>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    // this will set up the instantiation for other tests
    fn do_instantiate(mut deps: DepsMut) -> TokenInfoResponse {
        let instantiate_msg = InstantiateMsg {
            name: "Auto Gen".to_string(),
            symbol: "AUTO".to_string(),
            decimals: 6,
        };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let meta = query_token_info(deps.as_ref()).unwrap();
        assert_eq!(
            meta,
            TokenInfoResponse {
                name: "Auto Gen".to_string(),
                symbol: "AUTO".to_string(),
                decimals: 6,
                total_supply: Uint128::zero(),
            }
        );
        meta
    }

    mod instantiate {
        use super::*;

        #[test]
        fn basic() {
            let mut deps = mock_dependencies(&[]);
            let amount = Uint128::new(11223344);
            do_instantiate(deps.as_mut());

            let msg = ExecuteMsg::Mint {
                recipient: "addr0000".into(),
                amount: amount,
            };

            let info = mock_info("creator", &[]);
            let env = mock_env();

            let res = execute(deps.as_mut(), env, info, msg).unwrap();
            assert_eq!(0, res.messages.len());

            assert_eq!(
                query_token_info(deps.as_ref()).unwrap(),
                TokenInfoResponse {
                    name: "Auto Gen".to_string(),
                    symbol: "AUTO".to_string(),
                    decimals: 6,
                    total_supply: amount,
                }
            );
            assert_eq!(
                get_balance(deps.as_ref(), "addr0000"),
                Uint128::new(11223344)
            );
        }

        #[test]
        fn blocked() {
            let mut deps = mock_dependencies(&[]);
            let amount = Uint128::from(11223344u128);
            do_instantiate(deps.as_mut());

            // Mint to addr0000 from creator.
            let msg = ExecuteMsg::Mint {
                recipient: "addr0000".into(),
                amount: amount,
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();

            let res = execute(deps.as_mut(), env, info, msg).unwrap();
            assert_eq!(0, res.messages.len());

            // Block addr0000 from creator.
            let msg = ExecuteMsg::AddToBlockedList {
                address: "addr0000".into(),
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();

            let res = execute(deps.as_mut(), env, info, msg).unwrap();
            assert_eq!(0, res.messages.len());

            // Attempt a transfer from addr0000 to addr0001.
            let msg = ExecuteMsg::Transfer {
                recipient: "addr0001".into(),
                amount: Uint128::from(1000000u128),
            };
            let info = mock_info("addr0000", &[]);
            let env = mock_env();

            // Ensure transfer was blocked.
            let err = execute(deps.as_mut(), env, info, msg.clone()).unwrap_err();
            assert_eq!(err, ContractError::Blocked {});

            // Unblock addr0000.
            let unblock_msg = ExecuteMsg::RemoveFromBlockedList {
                address: "addr0000".into(),
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();

            let res = execute(deps.as_mut(), env, info, unblock_msg).unwrap();
            assert_eq!(0, res.messages.len());

            // Attempt a transfer from addr0000 to addr0001.
            let info = mock_info("addr0000", &[]);
            let env = mock_env();

            // Ensure transfer was successful.
            let res = execute(deps.as_mut(), env, info, msg.clone()).unwrap();
            assert_eq!(0, res.messages.len());
            assert_eq!(
                get_balance(deps.as_ref(), "addr0001"),
                Uint128::new(1000000)
            );
        }

        #[test]
        fn queries_work() {
            let mut deps = mock_dependencies(&[]);
            let env = mock_env();
            do_instantiate(deps.as_mut());
            let data = query(
                deps.as_ref(),
                env,
                QueryMsg::IsBlocked {
                    address: String::from("addr0000"),
                },
            )
            .unwrap();
            let loaded: BlockedResponse = from_binary(&data).unwrap();
            assert_eq!(loaded.blocked, false);

            // Block addr0000 from creator.
            let msg = ExecuteMsg::AddToBlockedList {
                address: "addr0000".into(),
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();

            let res = execute(deps.as_mut(), env, info, msg).unwrap();
            assert_eq!(0, res.messages.len());

            let env = mock_env();
            let data = query(
                deps.as_ref(),
                env,
                QueryMsg::IsBlocked {
                    address: String::from("addr0000"),
                },
            )
            .unwrap();
            let loaded: BlockedResponse = from_binary(&data).unwrap();
            assert_eq!(loaded.blocked, true);
        }
    }
}
