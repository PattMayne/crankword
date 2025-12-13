/* 
 * 
 * 
 * 
 * 
 * ====================================
 * ====================================
 * ===============      ===============
 * ===============  IO  ===============
 * ===============      ===============
 * ====================================
 * ====================================
 * 
 * 
 * 
 * 
*/


use serde::{ Deserialize, Serialize };
use reqwest::Client;
use anyhow::{ Result, anyhow };

use crate::{
    db, auth,
    auth_code_shared::{ 
        AuthCodeError,
        AuthCodeRequest,
        AuthCodeResponse,
        AuthCodeSuccess
    }
};


pub async fn check_auth_code(
    client_auth_data: AuthCodeRequest,
) -> anyhow::Result<AuthCodeSuccess> {
        // Use a reqwest Client for POST request
    let client: Client = Client::new();
    let response: AuthCodeResponse = client
        .post("http://auth.localhost.test:3000/ext_auth/verify_auth_code") // put this in resources file
        .json(&client_auth_data)
        .send()
        .await?
        .json::<AuthCodeResponse>()
        .await?;

    match response {
        AuthCodeResponse::Ok(success) => { Ok(success) }
        AuthCodeResponse::Err(err) => anyhow::bail!(err.message)
    }
}