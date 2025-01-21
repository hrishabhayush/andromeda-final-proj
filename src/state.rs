use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReviewMetadata {
    pub total_count: u32,
    pub average_rating: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Dispute {
    pub service_id: String,
    pub disputant: Addr,
    pub description: String,
    pub resolution: Option<String>,
}

pub const REVIEW_METADATA: Map<String, ReviewMetadata> = Map::new("review_metadata");
pub const PURCHASES: Map<String, Vec<Addr>> = Map::new("purchases");
pub const SERVICES: Map<String, Service> = Map::new("services");
pub const REVIEWS: Map<String, Vec<Review>> = Map::new("reviews");
pub const DISPUTE: Map<String, Vec<Dispute>> = Map::new("disputes");