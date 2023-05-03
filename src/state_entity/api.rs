use std::time::Instant;

use log::info;

use crate::{shared::{StateEntityFeeInfoAPI, UserID, DepositMsg1}, wallet::Wallet, utils::error::CError};

// pub fn get_statechain_fee_info() -> Result<StateEntityFeeInfoAPI> {
pub fn get_statechain_fee_info() -> StateEntityFeeInfoAPI{
    
    let value = reqwest::blocking::get("http://127.0.0.1:8000/info/fee")
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(value.as_str()).unwrap()
}

pub fn session_init(proof_key: &String) -> UserID {

    let start = Instant::now();
    
    let client = reqwest::blocking::Client::new();

    let body = DepositMsg1 {
        auth: "auth".to_string(),
        proof_key: proof_key.to_owned(),
    };

    let path = "deposit/init";
    let url = format!("http://127.0.0.1:8000/{}", path); 

    let request_builder = client.post(url);
    let response = request_builder.json(&body).send().unwrap();

    if response.content_length().unwrap() > 1000000 {
        panic!("Response too large");
    }

    let text = response.text().unwrap();

    if text.contains(&String::from("Error: ")) {
        panic!("{}", CError::StateEntityError(text));
    }

    info!("(req {}, took: {})", path, start.elapsed().as_secs_f32());
    serde_json::from_str(text.as_str()).unwrap()
    
}