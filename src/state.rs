use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Service {
    pub service_id: String,
    pub description: String,
    pub price: u128,
    pub category: String,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Review {
    pub service_id: String,
    pub reviewer: Addr,
    pub rating: u8,
    pub feedback: String,
}

pub const SERVICES: Map<String, Service> = Map::new("services");
pub const REVIEWS: Map<String, Vec<Review>> = Map::new("reviews");
