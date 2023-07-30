use bdk::{bitcoin, database, MemoryDatabase, Network};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;

    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);

    let wallet = bdk::Wallet::new(
        descriptor.into(),
        None,
        bdk::bitcoin::Network::Testnet,
        MemoryDatabase,
    );

    Ok(())
}
