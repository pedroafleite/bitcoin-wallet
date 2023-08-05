use bdk::{bitcoin::Network, database::SqliteDatabase, Wallet};
use std::path::Path;

fn setup() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").ok();
    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;
    Ok(descriptor)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = setup()?;

    let my_path: &Path = Path::new("bitcoin_wallet.db");

    let _wallet: Result<Wallet<SqliteDatabase>, bdk::Error> = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new(&my_path),
    );

    let address: Result<bdk::wallet::AddressInfo, bdk::Error> =
        _wallet?.get_address(bdk::wallet::AddressIndex::New);
    dbg!(address)?;

    Ok(())
}
