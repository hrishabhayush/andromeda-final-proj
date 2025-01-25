# Andromeda Decentralized Skills MarketPlace

**This repo is a variant of the [CosmWasm starter template](https://github.com/CosmWasm/cw-template).**

## Documentation

To see what's involved in making an ADO check out our documentation [here](https://docs.andromedaprotocol.io/andromeda/creating-an-ado/getting-started).

# Skills Marketplace

Skills Marketplace is a smart contract built using CosmWasm that allows users to list services, purchase services, leave reviews, and resolve disputes. This contract is designed to facilitate a decentralized marketplace for various skills and services. The marketplace allows freelancers to sell their expertise and buyers to list their projects on the platform for these freelancers. 

## Features

- **List Services**: Users can list their services with a description, price, and category.
- **Purchase Services**: Users can purchase listed services.
- **Leave Reviews**: Users can leave reviews for purchased services.
- **Resolve Disputes**: Disputes can be resolved by the service provider or an arbitrator.

## Contract Structure

### Messages

#### ExecuteMsg

- `ListService { service_id, description, price, category }`
- `PurchaseService { service_id, buyer }`
- `LeaveReview { service_id, rating, feedback }`
- `ResolveDispute { service_id, resolution }`

#### QueryMsg

- `GetServiceDetails { service_id }`
- `ListServices { category }`

### Responses

- `ServiceDetailsResponse`
- `ListServicesResponse`
- `ServiceSummary`
- `ProviderReviewsResponse`
- `ReviewSummary`

## Usage

### Instantiate

To instantiate the contract, use the following message:

```json
{
  "instantiate": {
    "kernel_address": "your_kernel_address",
    "owner": "your_owner_address"
  }
}
```

### Execute 

To execute various actions, use the following messages:

```json
{
  "list_service": {
    "service_id": "unique_service_id",
    "description": "service_description",
    "price": 100,
    "category": "service_category"
  }
}
```

### Purchase Service 

To purchase a service from a freelancer, the message is:

```json
{
  "purchase_service": {
    "service_id": "unique_service_id",
    "buyer": "buyer_address"
  }
}
```

### Leave Review

When a buyer wants to leave a review for a freelancer or a service, they use the following message:

```json
{
  "leave_review": {
    "service_id": "unique_service_id",
    "rating": 5,
    "feedback": "Great service!"
  }
}
```

### Resolve dispute

In case of any dispute, the message is:

```json
{
  "resolve_dispute": {
    "service_id": "unique_service_id",
    "resolution": "resolved"
  }
}
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

## Development 

### Prerequisites 

- Rust
- Cargo 
- CosmWasm

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

## License 

This project is licensed under the MIT License. 