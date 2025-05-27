use reqwest;
use reqwest::Client;
use serde_derive::{Deserialize};


static API_URL: String = std::env!("RANDOMI_API_URL");


#[derive(Deserialize)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub premium: bool,
    pub suspended: bool,
}


pub async fn get_account(token: &String) -> Option<Account> {
    let client = Client::default();
    let authorization: String = "Bearer ".to_owned() + token;
    let resp = client.get(String::from(&API_URL) + "/account")
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .send().await;

    match resp {
        Ok(resp) => {
            resp.json().await.unwrap_or(None)
        },
        Err(_) => None
    }
}
