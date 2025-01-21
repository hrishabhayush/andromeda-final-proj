use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ServiceDetailsResponse {
    pub service_id: String,
    pub description: String,
    pub price: u128,
    pub category: String,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListServicesResponse {
    pub services: Vec<ServiceSummary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ServiceSummary {
    pub service_id: String,
    pub description: String,
    pub price: u128,
    pub category: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProviderReviewsResponse {
    pub provider_id: Addr,
    pub reviews: Vec<ReviewSummary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReviewSummary {
    pub service_id: String,
    pub rating: u8,
    pub feedback: String,
}
