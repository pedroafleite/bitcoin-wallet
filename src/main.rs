fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;

    println!("Descriptor: {}", descriptor);
    // dbg!(descriptor);

    let _wallet = bdk::Wallet::new(
        &descriptor.clone(),
        None,
        bdk::bitcoin::Network::Testnet,
        bdk::database::MemoryDatabase::default(),
    );

    Ok(())
}
