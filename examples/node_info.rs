use std::env;

use dotenvy::dotenv;
use phoenixd_rs::Phoenixd;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let phoenixd = Phoenixd::new(
        &env::var("API_KEY").expect("API key not set"),
        &env::var("API_URL").expect("Api url not set"),
    )
    .unwrap();

    let r = phoenixd.get_node_info().await.unwrap();

    println!("{:?}", r);

    let r = phoenixd.get_balance().await.unwrap();

    println!("{:?}", r);

    let r = phoenixd.estimate_liquidity_fees(1_000_000).await.unwrap();

    println!("{:?}", r);

    let r = phoenixd.list_channels().await.unwrap();

    println!("{:?}", r);

    let r = phoenixd.get_ln_address().await.unwrap();

    println!("{:?}", r);
}
