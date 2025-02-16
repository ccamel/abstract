# Lending Market Adapter Module

The Lending Market Adapter Module provides a unified interface to interact with various lending and borrowing markets (sometimes called money markets) across the Cosmos ecosystem. By abstracting the differences between various lending markets, it allows developers to interact with any lending market using a standard interface, streamlining the development process and ensuring compatibility across various lending platforms.

## Features

- **Deposit**: Deposit funds for lending
- **Withdraw**: Withdraw lent funds.
- **Provide Collateral**:  Deposit collateral to borrow against
- **Withdraw Collateral**: Withdraw collateral to borrow against
- **Borrow**: Borrow funds from the lending market
- **Repay**: Repay funds to the lending market

```admonish info
Note that each one of these actions supports both ANS and raw variants, meaning that you can use both human-readable and explicit asset denominations.
```

## Supported Lending Markets

The following lending markets are currently supported:

- Mars (Osmosis, Neutron)
- Kujira Ghost (Kujira)

If you would like to request support for an additional lending market, please create a GitHub issue or reach out to us on Discord.

## Installation

To use the Lending Market Adapter Module in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
abstract-money-market-adapter = { git = "https://github.com/AbstractSDK/abstract.git", tag="v0.21.1", default-features = false }
```

## Usage with the Abstract SDK

To interact with a lending market, you first need to retrieve the lending market using the Moneymarket Api. Here's a basic example in Rust:

```rust
// Retrieve the money_market
use abstract_money_market_adapter::api::MoneyMarketInterface;
...

let money_market_name = "mars".to_string();
let deposit_asset = Asset::native("ujuno", 12345u128);

// Using the raw (non ANS-enabled) lending market
let money_market = app.money_market(deps.as_ref(), money_market_name);
let deposit_msg = money_market.deposit(deposit_asset);
```

## Limitation

The Lending Market adapter provides easy ways of interacting with lending markets. However, some errors can appear without the adapter catching them:

- The lending market can have deposit limits enabled which may be crossed when using this adapter.
- The lending market may not have liquidity available to borrow funds.
- The lending market may not have liquidity available to withdraw deposited funds from
- The user may not be able to withdraw collateral because they are borrowing too many funds.

All those errors and more have to be handled directly by the developers integrating this adapter.

## Why Use the Lending Market Adapter?

### Simplified Development

By using the Adapter, developers can bypass the intricacies of each individual platform. This means less time spent on understanding and integrating with each lending market's unique API, and more time focusing on building core functionalities.

### Flexibility

The Lending Market Adapter ensures that your application remains flexible. If a new lending market emerges or if there are changes to an existing one, your application can easily adapt without undergoing major overhauls.

### Use Cases

- **Rapid Prototyping**: Quickly build and test applications on top of various lending markets without the need for multiple integrations.
- **Cross-Dex Applications**: Build applications that leverage multiple lending markets simultaneously, offering users more options and better rates.
- **Future-Proofing**: Ensure your application remains compatible with future lending markets that emerge in the Cosmos ecosystem.

## Documentation

- **Lending Market Interface**: For a detailed look at the lending market interface, refer to the [Rust trait interface](https://github.com/AbstractSDK/abstract/tree/main/modules/contracts/adapters/moneymarket/src/api.rs#L43). #TODO, fix this will be broken

- **Adapters Documentation**: Comprehensive information about adapters can be found in the [official documentation](https://docs.abstract.money/3_framework/7_module_types.html#adapters).

## Contributing

If you have suggestions, improvements, new lending markets, or want to contribute to the project, we welcome your input on GitHub.
