use andromeda_std::{andr_exec, andr_instantiate, andr_query};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
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
        buyer: String,
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

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
