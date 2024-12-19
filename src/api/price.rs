use super::Error;

use super::structs::{DateTime, Request};

use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
struct PriceRes {data: Price}
#[derive(Deserialize)]
struct Price {amount: String, currency: String}
#[derive(Deserialize)]
struct Spot {amount: String, currency: String, base: String}
#[derive(Deserialize)]
struct SpotRes {
    data: Spot,
}

const PRICE_URL: &str = "https://api.coinbase.com/v2/prices/BTC-USD/buy";
const PRICES_URL: &str = "https://api.coinbase.com/v2/prices/BTC-USD/spot";

#[derive(Clone, Debug)]
pub struct PriceGetter {}

impl PriceGetter {
    pub async fn get(timestamp: Option<&DateTime>) -> Result<f64, Error> {
        Ok(match timestamp {
            None => {
                let res = Request::get(PRICE_URL).await?.json::<Value>().await?;
                let price_res: PriceRes = serde_json::from_value(res)?;
                price_res.data.amount.parse::<f64>()?
            },
            Some(timestamp) => {
                let date = timestamp.format("%Y-%m-%d");
                let url = format!("{}?date={}", PRICES_URL, date);
                let reqwest = Request::get(&url).await?;
                let value: serde_json::Value = reqwest.json().await?;
                let spot_res: SpotRes = serde_json::from_value(value.clone()).or(
                    Err(Error::err("Fetching price from coinbases api",
                        &serde_json::to_string(&value)?
                    ))
                )?;
                spot_res.data.amount.parse::<f64>()?
            }
        })
    }
}
