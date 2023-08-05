// use bdk::{bitcoin::Network, database::SqliteDatabase, Wallet};
// use std::path::Path;

// fn setup() -> Result<String, Box<dyn std::error::Error>> {
//     dotenv::from_filename(".env").ok();
//     let descriptor = std::env::var("WALLET_DESCRIPTOR")?;
//     Ok(descriptor)
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let descriptor = setup()?;

//     let my_path: &Path = Path::new("bitcoin_wallet.db");

//     let _wallet: Result<Wallet<SqliteDatabase>, bdk::Error> = Wallet::new(
//         &descriptor,
//         None,
//         Network::Testnet,
//         SqliteDatabase::new(&my_path),
//     );

//     let address: Result<bdk::wallet::AddressInfo, bdk::Error> =
//         _wallet?.get_address(bdk::wallet::AddressIndex::New);
//     dbg!(address)?;

//     Ok(())
// }

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    // axum::serve(listener, app).await.unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
