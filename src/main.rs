fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;  

    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);

    Ok(())

}
