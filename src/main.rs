use axum::{response::Html, routing::get, Router};
use bdk::{bitcoin::Network, database::SqliteDatabase, Wallet};
use std::net::SocketAddr;
use std::path::Path;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::{StatusCode};
use axum::Json;

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

fn setup() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").ok();
    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;
    Ok(descriptor)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    let response = AddressResponse{
        address: "test".to_string(),
        index: 0
    };

    Ok(Json(response))
}

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}