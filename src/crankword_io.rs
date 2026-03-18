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
 * Functions for interacting with the auth_site
 * 
 * 
*/

use reqwest::Client;

use crate::{
    auth_code_shared::{ 
        AuthCodeRequest,
        AuthCodeResponse,
        AuthCodeSuccess,
        RefreshCheckResponse,
        RefreshCheckSuccess,
        RefreshCheckRequest,
    }
};


pub async fn check_auth_code(
    client_auth_data: AuthCodeRequest,
) -> anyhow::Result<AuthCodeSuccess> {
    // Use a reqwest Client for POST request
    let client: Client = Client::new();
    let response: reqwest::Response = client
        //.post("http://auth.localhost.test:3000/ext_auth/verify_auth_code")
        .post("https://crankade.com/ext_auth/verify_auth_code") // todo: put this in resources file
        .json(&client_auth_data)
        .send()
        .await?;

    let bytes: actix_web::web::Bytes = response.bytes().await?;

    // Try to parse as JSON
    let parsed: Result<AuthCodeResponse, _> = serde_json::from_slice(&bytes);

    // If failed to parse, print bytes as string
    if let Err(error) = parsed {
        println!("Failed to parse: {:?}", error);
        println!("Raw response: {}", String::from_utf8_lossy(&bytes));
        anyhow::bail!("Failed to parse AuthCodeResponse: {:?}", error);
    }
    
    match parsed.unwrap() {
        AuthCodeResponse::Ok(success) => Ok(success),
        AuthCodeResponse::Err(err) => anyhow::bail!(err.message),
    }
}

/**
 * When the user's JWT runs out we get their refresh_token and send to the auth_app,
 * which checks that it's valid and non-expired.
 */
pub async fn check_refresh_code(refresh_request: &RefreshCheckRequest)
    -> anyhow::Result<RefreshCheckSuccess>
{
    // Use a reqwest Client for POST request
    let client: Client = Client::new();
    let response: RefreshCheckResponse = client
        .post("https://crankade.com/ext_auth/check_refresh") // todo: put this in resources file
        //.post("http://auth.localhost.test:3000/ext_auth/check_refresh") // put this in resources file
        .json(&refresh_request)
        .send()
        .await?
        .json::<RefreshCheckResponse>()
        .await?;

    match response {
        RefreshCheckResponse::Ok(success) => { Ok(success) }
        RefreshCheckResponse::Err(err) => anyhow::bail!(err.message)
    }
}

