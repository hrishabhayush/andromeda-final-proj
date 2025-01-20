use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg,
    ado_contract::ADOContract,
    common::{actions::call_action, context::ExecuteContext},
    error::ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:skills-marketplace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract = ADOContract::default();

    let resp = contract.instantiate(
        deps.storage,
        env,
        deps.api,
        &deps.querier,
        info.clone(),
        BaseInstantiateMsg {
            ado_type: CONTRACT_NAME.to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    Ok(resp
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    if let ExecuteMsg::AMPReceive(pkt) = msg {
        ADOContract::default().execute_amp_receive(ctx, pkt, handle_execute)
    } else {
        handle_execute(ctx, msg)
    }
}

pub fn handle_execute(mut ctx: ExecuteContext, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let action_response = call_action(
        &mut ctx.deps,
        &ctx.info,
        &ctx.env,
        &ctx.amp_ctx,
        msg.as_ref(),
    )?;

    let res = match msg {
        ExecuteMsg::ListService {
            service_id,
            description,
            price,
            category,
        } => list_service(ctx, service_id, description, price, category),
        ExecuteMsg::PurchaseService { service_id, buyer } => {
            purchase_service(ctx, service_id, buyer)
        }
        ExecuteMsg::LeaveReview {
            service_id,
            rating,
            feedback,
        } => leave_review(ctx, service_id, rating, feedback),
        ExecuteMsg::ResolveDispute {
            service_id,
            resolution,
        } => resolve_dispute(ctx, service_id, resolution),
        _ => ADOContract::default().execute(ctx, msg),
    }?;

    Ok(res
        .add_submessages(action_response.messages)
        .add_attributes(action_response.attributes)
        .add_events(action_response.events))
}

fn list_service(
    ctx: ExecuteContext,
    service_id: String,
    description: String,
    price: u128,
    category: String,
) -> Result<Response, ContractError> {
    // Implement service listing logic (e.g., store in state)
    Ok(Response::new()
        .add_attribute("action", "list_service")
        .add_attribute("service_id", service_id)
        .add_attribute("description", description)
        .add_attribute("price", price.to_string())
        .add_attribute("category", category))
}

fn purchase_service(
    ctx: ExecuteContext,
    service_id: String,
    buyer: String,
) -> Result<Response, ContractError> {
    // Implement purchase service logic here
    Ok(Response::new()
        .add_attribute("action", "purchase_service")
        .add_attribute("service_id", service_id)
        .add_attribute("buyer", buyer))
}

fn leave_review(
    ctx: ExecuteContext,
    service_id: String,
    rating: u8,
    feedback: String,
) -> Result<Response, ContractError> {
    // Implement review logic (e.g., store review in state)
    Ok(Response::new()
        .add_attribute("action", "leave_review")
        .add_attribute("service_id", service_id)
        .add_attribute("rating", rating.to_string()))
}

fn resolve_dispute(
    ctx: ExecuteContext,
    service_id: String,
    resolution: String,
) -> Result<Response, ContractError> {
    // Implement dispute resolution logic
    Ok(Response::new()
        .add_attribute("action", "resolve_dispute")
        .add_attribute("service_id", service_id)
        .add_attribute("resolution", resolution))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        _ => ADOContract::default().query(deps, env, msg),
    }
}

#[cfg(test)]
mod tests {}
