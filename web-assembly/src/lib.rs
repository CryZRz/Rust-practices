use serde_json::{Map, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, window, Request, Response};

#[wasm_bindgen]
pub async fn get_coin_price(coin: String) -> Result<String, JsValue>{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.coingecko.com/api/v3/coins/{coin}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false");

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text().unwrap()).await.unwrap().as_string().unwrap();

    let price = parse_body(&text);
    Ok(price)
} 

pub fn parse_body(text: &str) -> String{
    let json: Map<String, Value> = serde_json::from_str(text).unwrap();
    json["market_data"]["current_price"]["usd"].to_string()
}