use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg,
    ado_contract::ADOContract,
    common::{actions::call_action, context::ExecuteContext},
    error::ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{
        Review, ReviewMetadata, Service, DISPUTE, PURCHASES, REVIEWS, REVIEW_METADATA, SERVICES,
    },
};

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
    let sender = ctx.info.sender;

    // Ensure the service does not already exist
    if SERVICES.has(ctx.deps.storage, service_id.clone()) {
        return Err(ContractError::Std(StdError::generic_err(
            "Service already exists",
        )));
    }

    let service = Service {
        service_id: service_id.clone(),
        description: description.clone(),
        price,
        category: category.clone(),
        owner: sender.clone(),
    };

    SERVICES.save(ctx.deps.storage, service_id.clone(), &service)?;

    Ok(Response::new()
        .add_attribute("action", "list_service")
        .add_attribute("service_id", service_id)
        .add_attribute("description", description.clone())
        .add_attribute("price", price.to_string())
        .add_attribute("category", category.clone())
        .add_attribute("owner", sender))
}

fn purchase_service(
    ctx: ExecuteContext,
    service_id: String,
    buyer: Addr,
) -> Result<Response, ContractError> {
    // Check if the service exists
    let _service = SERVICES.load(ctx.deps.storage, service_id.clone())?;

    PURCHASES.update(
        ctx.deps.storage,
        service_id.clone(),
        |purchases| -> StdResult<_> {
            let mut purchases = purchases.unwrap_or_default();
            purchases.push(buyer.clone());
            Ok(purchases)
        },
    )?;

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
    let sender = ctx.info.sender;

    // Validate the rating
    if rating > 5 {
        return Err(ContractError::Std(StdError::generic_err(
            "Rating must be between 0 and 5",
        )));
    }

    let review = Review {
        service_id: service_id.clone(),
        reviewer: sender.clone(),
        rating,
        feedback,
    };

    REVIEWS.update(
        ctx.deps.storage,
        service_id.clone(),
        |reviews| -> StdResult<_> {
            let mut reviews = reviews.unwrap_or_default();
            reviews.push(review.clone());
            Ok(reviews)
        },
    )?;

    REVIEW_METADATA.update(
        ctx.deps.storage,
        service_id.clone(),
        |metadata| -> StdResult<_> {
            let mut metadata = metadata.unwrap_or(ReviewMetadata {
                total_count: 0,
                average_rating: 0.0,
            });
            metadata.total_count += 1;
            metadata.average_rating =
                ((metadata.average_rating * (metadata.total_count - 1) as f32) + rating as f32)
                    / metadata.total_count as f32;
            Ok(metadata)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "leave_review")
        .add_attribute("service_id", service_id)
        .add_attribute("rating", rating.to_string())
        .add_attribute("sender", sender))
}

fn resolve_dispute(
    ctx: ExecuteContext,
    service_id: String,
    resolution: String,
) -> Result<Response, ContractError> {
    let sender = ctx.info.sender;

    // Update the dispute resolution
    DISPUTE.update(
        ctx.deps.storage,
        service_id.clone(),
        |disputes| -> StdResult<_> {
            let mut disputes = disputes.unwrap_or_default();

            // Find the first unresolved dispute
            if let Some(dispute) = disputes.iter_mut().find(|d| d.resolution.is_none()) {
                dispute.resolution = Some(resolution.clone());
            } else {
                return Err(StdError::generic_err(
                    "No unresolved disputes for this service",
                ));
            }

            Ok(disputes)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "resolve_dispute")
        .add_attribute("service_id", service_id)
        .add_attribute("sender", sender)
        .add_attribute("resolution", resolution))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        _ => ADOContract::default().query(deps, _env, msg),
    }
}

#[cfg(test)]
mod tests {}
