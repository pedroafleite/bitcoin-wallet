# bitcoin-wallet

In the Bitcoin Development Kit (BDK), wpkh stands for “Witness Public Key Hash”. It is a descriptor that represents a pay-to-witness-public-key-hash (P2WPKH) output script. The wpkh descriptor is used to create a new wallet in BDK. To create a wallet using the BDK descriptor, you can use the `bdk::Wallet::new` function. This is the example used in the code:

`let _wallet = bdk::Wallet::new(
    &descriptor.clone(),
    None,
    bdk::bitcoin::Network::Testnet,
    bdk::database::MemoryDatabase::default(),
);`

The `bdk::Wallet::new` function takes four arguments:

- descriptor: A string that represents the descriptor for the wallet.
- change_descriptor: An optional string that represents the descriptor for the change address. If this argument is None, then a default change descriptor will be used.
- network: An enum that represents the Bitcoin network to use (e.g., `bdk::bitcoin::Network::Testnet`).
- database: An object that implements the `bdk::database::BatchDatabase`` trait, which is used to store the wallet’s data.
