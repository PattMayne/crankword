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
        SendVerificationEmailRequest,
        SendVerificationEmailResponse
    },
    auth::{ UserReqData }
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


/**
 * This should only work when user is logged in
 */
pub async fn send_email_verification_request(
    user_req_data: &UserReqData
) -> anyhow::Result<SendVerificationEmailResponse> {

    let user_id: i32 = match (user_req_data.logged_in, user_req_data.id) {
        (true, Some(id)) => id,
        _ => anyhow::bail!("USER NOT LOGGED IN")
    };

    let client_id: String =
        match std::env::var("CLIENT_ID") {
            Ok(secret) => secret,
            Err(_e) => {
                eprintln!("ERROR: NO CLIENT ID.");
                anyhow::bail!("NO CLIENT ID")
            }
        };

    let client_secret: String =
        match std::env::var("CLIENT_SECRET") {
            Ok(secret) => secret,
            Err(_e) => {
                eprintln!("ERROR: NO CLIENT SECRET.");
                anyhow::bail!("ERROR: NO CLIENT SECRET.")
            }
        };

    let email_req: SendVerificationEmailRequest =
        SendVerificationEmailRequest{
            client_id, client_secret, user_id
        };

    // Use a reqwest Client for POST request
    let client: Client = Client::new();
    let response: SendVerificationEmailResponse = client
        .post("https://crankade.com/ext_auth/re_ver_email") // todo: put this in resources file
        //.post("http://auth.localhost.test:3000/ext_auth/check_refresh") // put this in resources file
        .json(&email_req)
        .send()
        .await?
        .json::<SendVerificationEmailResponse>()
        .await?;

    Ok(response)
}
