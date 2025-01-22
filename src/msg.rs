use crate::responses::{ListServicesResponse, ProviderReviewsResponse, ServiceDetailsResponse};
use andromeda_std::{andr_exec, andr_instantiate, andr_query};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub platform_fee: u128,
}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    ListService {
        service_id: String,
        description: String,
        price: u128,
        category: String,
    },
    PurchaseService {
        service_id: String,
        buyer: Addr,
    },
    LeaveReview {
        service_id: String,
        rating: u8,
        feedback: String,
    },
    ResolveDispute {
        service_id: String,
        resolution: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ServiceDetailsResponse)]
    GetServiceDetails { service_id: String },
    #[returns(ListServicesResponse)]
    ListServices { category: Option<String> },
    #[returns(ProviderReviewsResponse)]
    GetProviderReviews { provider_id: String },
}
