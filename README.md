# bitcoin-wallet

In the Bitcoin Development Kit (BDK), wpkh stands for “Witness Public Key Hash”. It is a descriptor that represents a pay-to-witness-public-key-hash (P2WPKH) output script. The wpkh descriptor is used to create a new wallet in BDK. To create a wallet using the BDK descriptor, you can use the `bdk::Wallet::new` function. This is the example used in the code:

```rust
let _wallet = bdk::Wallet::new(
    &descriptor.clone(),
    None,
    bdk::bitcoin::Network::Testnet,
    bdk::database::MemoryDatabase::default(),
);
```

The `bdk::Wallet::new` function takes four arguments:

- descriptor: A string that represents the descriptor for the wallet.
- change_descriptor: An optional string that represents the descriptor for the change address. If this argument is None, then a default change descriptor will be used.
- network: An enum that represents the Bitcoin network to use (e.g., `bdk::bitcoin::Network::Testnet`).
- database: An object that implements the `bdk::database::BatchDatabase`` trait, which is used to store the wallet’s data.

In the Bitcoin Development Kit (BDK), a descriptor is a compact and semi-standard way to easily encode, or “describe”, how scripts (and subsequently, addresses) of a wallet should be generated. Descriptors can be especially helpful when working with multisigs or even more complex scripts, where the structure of the script itself is not trivial. They are a big step forward in making wallets more portable across different tools and apps, because for the first time they create a common language to describe a full bitcoin script that developers can use and integrate in their software. The ecosystem around descriptors is still very much in its early stage, but they are starting to see some adoption in Bitcoin Core and other projects. BDK aims to produce the first “Native Descriptor” Bitcoin library that can be used by developers to build their own “Native Descriptor Wallets”.

BDK extends the capabilities of rust-miniscript by introducing the concept of an ExtendedDescriptor: it represents a descriptor that contains one or more “derivable keys” like xpubs or xprvs and can be “derived” from a normal Descriptor by deriving every single one of its keys. The bdk::descriptor module provides types and functions for working with descriptors in BDK.
