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
        .post("http://auth.localhost.test:3000/ext_auth/check_refresh") // put this in resources file
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

