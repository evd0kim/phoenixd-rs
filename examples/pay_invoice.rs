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

    let invoice = &env::var("BOLT11").expect("BOLT11 key not set");

    let pay_response = phoenixd.pay_bolt11_invoice(invoice, None).await.unwrap();

    println!("{:?}", pay_response);
}
