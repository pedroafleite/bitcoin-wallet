use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor: String = std::env::var("WALLET_DESCRIPTOR")?;

    println!("Descriptor: {}", descriptor);

    let _wallet: Result<Wallet<MemoryDatabase>, bdk::Error> = Wallet::new(
        &descriptor, // &descriptor.clone();
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    );

    let address: Result<bdk::wallet::AddressInfo, bdk::Error> =
        _wallet?.get_address(bdk::wallet::AddressIndex::New);

    dbg!(address)?;

    Ok(())
}
