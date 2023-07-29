use std::env;

fn main() {
    println!("Hello, world!");
    dotenv::from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();  

    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);

}
