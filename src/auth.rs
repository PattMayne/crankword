use jsonwebtoken::{
    encode, Header, EncodingKey, decode, dangerous::insecure_decode,
    DecodingKey, Validation, Algorithm, errors::{ Error, ErrorKind} };
use serde::{ Serialize, Deserialize };
use time::{ Duration, OffsetDateTime };
use actix_web::{ HttpMessage, HttpRequest, cookie::{Cookie, SameSite}};
use rand::{distr::Alphanumeric, Rng};
use std::fmt;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};

use crate::utils::{self, SupportedLangs};

/*
 * 
 * 
 * ============================
 * ============================
 * =====                  =====
 * =====  AUTH FUNCTIONS  =====
 * =====                  =====
 * ============================
 * ============================
 * 
 * 
 * This is copied from auth_app.
 * We need some common library of structs and functions.
 * 
 * 
 * TOKEN USAGE AND STORAGE
 *
 * ACCESS TOKENS: JWTs (JSON Web Tokens)
 * -------------
 * -- We make our own JWTs here in the client app
 * -- short-lived (few minutes to an hour) (probably 20 minutes)
 * -- FRONT END STORAGE has TWO SCHEMES:
 * -- -- SPA: For single-page applications (game page), store in JavaScript memory in the front-end
 * -- -- -- in this scenario we send the JWT back for each API call which updates live game state
 * -- -- -- This is the scenario we will always use within the auth app
 * -- -- MPA: For clicking between pages (everywhere else), store in an HTTP-only cookie
 * -- -- -- backend sets this in the response. front-end JS cannot touch it or read it.
 * -- -- -- access token is then sent safely within headers
 * -- -- SWITCHING b/w schemes requires checking refresh token & sending new JWT
 * -- not stored in the backend at all
 * -- algorithmically verified in the backend
 * -- sent back for each request that requires being logged in
 * -- no need for sessions, as this is your ticket for each request
 * -- when expired, user must send refresh token (different token) to get a new access token
 * 
 * 
 * REFRESH TOKENS: Just a long random string
 * -------------
 * -- created on the backend and sent from auth_app to client
 * -- long-lived (several days to several weeks)
 * -- stored in HttpOnly Cookie in the front-end (protected against XXS)
 * -- stored in database (Users table) in the backend
 * -- Only sent to the backend when user needs a new JWT access token
 * -- backend verifies by checking received token against the one in the DB
 * -- when expired, user has to log in
 * -- logging in and registering generate refresh token
 *
 *
 * TO DO:
 */


 /* 
 * 
 * 
 * 
 * 
 * 
 * 
 * ===============================
 * ===============================
 * ==========           ==========
 * ==========  STRUCTS  ==========
 * ==========           ==========
 * ===============================
 * ===============================
 * 
 * and their implemented functions
 * 
 * 
 * 
 * 
 */

/* 
 * This holds that data that gets encoded into a JSON Web Token (JWT).
 * The user is "claiming" to be a certain identity.
 */ 
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    role: String,
    username: String,
    exp: usize, // expiration as a timestamp (seconds since epoch)
}

pub enum JwtVerification {
    Valid(Claims),
    Expired(Claims),
    Invalid
}

#[derive(Debug)]
pub enum AuthError {
    Jwt(jsonwebtoken::errors::Error),
    MissingJwtSecret,
}

/* 
 * Middleware will insert this struct into every request so the routes
 * know who they're dealing with.
 * The most basic data about a user, info we might casually need
 * on any page.
 */
#[derive(Clone)]
pub struct UserReqData {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub role: String, // guest, player, admin
    pub logged_in: bool,
    pub lang: utils::SupportedLangs,
}


/* 
 * 
 * ====================================
 * ====================================
 * =====                          =====
 * =====  STRUCT IMPLEMENTATIONS  =====
 * =====                          =====
 * ====================================
 * ====================================
 * 
*/


impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            AuthError::MissingJwtSecret => "Missing JWT Secret".to_owned(),
            AuthError::Jwt(err) => format!("JWT error: {}", err),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AuthError::Jwt(err) => Some(err),
            AuthError::MissingJwtSecret => None,
        }
    }
}

/**
 * User Data for middleware to put into the Request, for the route functions to use.
 * UserReqData is built from a Claims object (taken from the JWT).
 * If we have a claims struct, the user must be logged in, so send it in as Some<claims>.
 * Otherwise, send in None and we will make a generic Guest object.
 */
impl UserReqData {

    /**
     * If a Claims struct is available then build UserReqData from it.
     * Otherwise generate a generic guest struct.
     */
    pub fn new(claims_option: Option<Claims>) -> Self {
        match claims_option {
            Some(claims) => {
                UserReqData {
                    id: Some(claims.get_sub()),
                    username: Some(claims.get_username().to_owned()),
                    role: claims.get_role().to_owned(),
                    logged_in: true,
                    lang: utils::SupportedLangs::English,
                }
            },
            None => {
                UserReqData {
                    id: None,
                    username: None,
                    role: String::from("guest"),
                    logged_in: false,
                    lang: utils::SupportedLangs::English,
                }
            }
        }
    }

    pub fn get_role(&self) -> &String {
        &self.role
    }

    pub fn is_admin(&self) -> bool {
        &self.role == "admin"
    }

    pub fn lang_suffix(&self) -> &'static str {
        self.lang.suffix()
    }

    pub fn clone_lang(&self) -> SupportedLangs {
        self.lang.clone()
    }
}

/* functions for the Claims struct */
impl Claims {
    pub fn get_sub(&self) -> i32 { self.sub }
    pub fn get_role(&self) -> &String { &self.role }
    pub fn get_username(&self) -> &String { &self.username }
    pub fn get_exp(&self) -> usize { self.exp }
}



/**
 * Send in the request and we'll extract the UserReqData for you.
 * If it doesn't exist we'll assumed the user is a guest, and we will
 * make a new UserReqData for you.
 * The middleware already checked the jwt to get the user data.
 * This is where we retrieve the result of that check for each route.
 */
pub fn get_user_req_data(req: &HttpRequest) -> UserReqData {
    let guest_user: UserReqData = UserReqData::new(None);
    let extensions: std::cell::Ref<'_, actix_web::dev::Extensions> = req.extensions();

    // Get user data from req
    match extensions.get::<UserReqData>() {
        Some(user_data) => user_data,
        None => &guest_user
    }.to_owned()
}

/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
 * xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
 * xxxxxxxxxx          xxxxxxxxxx
 * xxxxxxxxxx  TOKENS  xxxxxxxxxx
 * xxxxxxxxxx          xxxxxxxxxx
 * xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
 * xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
 * 
 * JSON Web Tokens
 * Refresh Tokens
 * Build them, put them in cookies, verify them, etc.
 * 
 * 
 * 
 * 
 * 
 * 
 */

/**
 * JSON Web Token generator.
 * Take some info about the user to create a Claims struct,
 * along with the JWT secret,
 * and generate an encoded JWT String
 * to use as an access token for the user.
 */
pub fn generate_jwt(
    user_id: i32,
    username: String,
    role: String,
    //secret: &[u8]
) -> Result<String, AuthError> {
    // Set expiration for 1 hour from now
    let exp: usize = (OffsetDateTime::now_utc() + Duration::minutes(30))
        .unix_timestamp() as usize;

    let claims: Claims = Claims {
        sub: user_id,
        username,
        role,
        exp,
    };

    println!("Generating JWT");

    // Get JWT secret from env. Return err if missing.
    let jwt_secret: String = get_jwt_secret()
        .map_err(|_| AuthError::MissingJwtSecret)?;

    // secret exists in env variables. Encode and match the result.
    // Encoding includes the HS256 signature.
    let jwt_result: Result<String, Error> =
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes())
        );
    
    match jwt_result {
        Ok(jwt) => Ok(jwt),
        Err(_e) => Err(AuthError::MissingJwtSecret)
    }  
}



/**
 * Setting a cookie only works for browsing within the auth site
 * For external app authentication we will implement OAuth2
 */
pub fn build_token_cookie(token: String, name: String) -> Cookie<'static> {
    // WARNING: THIS MUST BE TRUE IN PROD. Change env variable
    let secure: bool = std::env::var("COOKIE_SECURE")
        .map(|value: String| value == "true")
        .unwrap_or(false);

    Cookie::build(name, token)
        .http_only(true)
        .secure(secure) 
        .same_site(SameSite::Lax)
        .path("/")
        .finish()
}



/**
 * Decode the jwt string, check it against the Claims struct.
 * If the JWT is expired, we will still return the Claims (using insecure_decode)
 * and the receiver must check the expiry date in the claims.
 * If all is well, return the Claims stuct in case we want to
 * use that data or check it against DB data.
 */
pub async fn verify_jwt(token: &str) -> JwtVerification {
    // get the jwt secret so we can decode the jwt string
    let secret: String = match get_jwt_secret() {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to retrieve JWT SECRET");
            return JwtVerification::Invalid;
        }
    };

    // HS256 algorithm matches the header default I use to encode
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => JwtVerification::Valid(token_data.claims), // good. send.
        Err(e) => match *e.kind() {
            ErrorKind::ExpiredSignature => {
                match insecure_decode::<Claims>(token) {
                    Ok(token_data) => {
                        JwtVerification::Expired(token_data.claims)
                    },
                    Err(_e) => JwtVerification::Invalid
                }
            },
            _ => JwtVerification::Invalid
        }
    }
    
}


// Get the JWT secret from env variables
pub fn get_jwt_secret() -> Result<String, std::env::VarError> {
    std::env::var("JWT_SECRET")
}

