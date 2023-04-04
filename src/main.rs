extern crate reqwest;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    price_change: String,
    first_id: i64,
    ask_qty: String,
    quote_volume: String,
    high_price: String,
    bid_qty: String,
    last_qty: String,
    low_price: String,
    weighted_avg_price: String,
    bid_price: String,
    count: i64,
    open_time: i64,
    open_price: String,
    volume: String,
    price_change_percent: String,
    symbol: String,
    prev_close_price: String,
    ask_price: String,
    close_time: i64,
    last_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
enum Ticker24hr {
    Ticker(Ticker),
    VectorTicker(Vec<Ticker>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let req = client.get("https://api.binance.com/api/v3/ticker/24hr")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;

    let data: Ticker24hr = serde_json::from_str(&req)?;
    println!("{:#?}", data);

    Ok(())
}