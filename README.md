# Skills Marketplace README

## Overview

The **Skills Marketplace** is a decentralized platform that facilitates seamless interactions between service providers and seekers. It provides a transparent and secure environment for listing services, purchasing services, leaving reviews, and resolving disputes.

---

## Features

### Core Functionalities
1. **Service Listing**
   - Service providers can list services with detailed attributes like description, price, and category.
   - Metadata is stored for efficient querying.

2. **Service Purchase**
   - Users can purchase services directly from providers, with the platform collecting a configurable platform fee.

3. **Review System**
   - Buyers can leave ratings and feedback on purchased services.
   - Aggregated review metadata, such as average ratings and total reviews, is maintained for better insights.

4. **Dispute Resolution**
   - Mechanisms for resolving disputes, with all resolutions securely stored.

### Query Functionalities
- Retrieve details about specific services.
- List services by category or retrieve all available services.
- Fetch reviews and ratings for service providers.

---

## Message Types

### InstantiateMsg
Used during contract initialization:
- `admin`: Address of the platform administrator.
- `platform_fee`: Fee percentage collected for each transaction.

### ExecuteMsg
Available actions:
- **ListService**: Add a new service with its details.
- **PurchaseService**: Purchase a service using the service ID and buyer's address.
- **LeaveReview**: Submit a rating and feedback for a service.
- **ResolveDispute**: Submit a resolution for a dispute.

### QueryMsg
Read-only operations:
- **GetServiceDetails**: Fetch detailed information for a service.
- **ListServices**: List services, optionally filtered by category.
- **GetProviderReviews**: Retrieve reviews for a specific provider.

---

## Data Structures

### Service
Represents a listed service:
```rust
pub struct Service {
    pub service_id: String,
    pub description: String,
    pub price: u128,
    pub category: String,
    pub owner: Addr,
}
```

### Review 
Details about feedback for services:
```rust 
pub struct Review {
    pub service_id: String,
    pub reviewer: Addr,
    pub rating: u8,
    pub feedback: String,
}
```

### ReviewMetaData
Aggregated review statistics to give an information about a freelancer
```rust
pub struct ReviewMetadata {
    pub total_count: u32,
    pub average_rating: f32,
}
```

### Dispute
Represents a dispute resolution process:
```rust
pub struct Dispute {
    pub service_id: String,
    pub disputant: Addr,
    pub description: String,
    pub resolution: Option<String>,
}
```

## Storage

### Maps
```rust
pub const SERVICES: Map<String, Service> = Map::new("services");
pub const REVIEWS: Map<String, Vec<Review>> = Map::new("reviews");
pub const REVIEW_METADATA: Map<String, ReviewMetadata> = Map::new("review_metadata");
pub const PURCHASES: Map<String, Vec<Addr>> = Map::new("purchases");
pub const DISPUTES: Map<String, Vec<Dispute>> = Map::new("disputes");
```

## Responses 

### ServiceDetailsResponse
Details about a specific service:
```rust
pub struct ServiceDetailsResponse {
    pub service_id: String,
    pub description: String,
    pub price: u128,
    pub category: String,
    pub owner: Addr,
}
```

### ListServicesResponse
List of summarized services:
```rust
pub struct ListServicesResponse {
    pub services: Vec<ServiceSummary>,
}
```

### ProviderReviewsResponse
Reviews and ratings for a freelancer:
```rust
pub struct ProviderReviewsResponse {
    pub provider_id: Addr,
    pub reviews: Vec<ReviewSummary>,
}
```

## Development

### Prerequisites
- Rust environment with `cargo` installed.
- CosmWasm development environment set up.

### Build Instructions
1. Clone the repository.
2. Navigate to the project directory.
3. Build the contract:
   ```bash
   cargo build --release
   ```

## Query 

To query the contract, use the following messages:

### Get Service details:

```json 
{
  "get_service_details": {
    "service_id": "unique_service_id"
  }
}
```

### List Services

```json
{
  "list_services": {
    "category": "service_category"
  }
}
```

### Building 

To build the contract, run: 

```sh
cargo build --release
```

### Testing 

To run test, use:

```sh
cargo test
```

### Formatting and Linting

To format the code properly, run:

```sh
cargo fmt
```

To lint the code, run:

```sh
cargo clippy -- -D warnings
```