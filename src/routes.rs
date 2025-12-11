use crate::db;

/* 
 * ====================
 * ====================
 * =====          =====
 * =====  ROUTES  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * Functions to be called when user request hits endpoints listed
 * in the main function.
 * 
 * 
*/
// why it gray?
use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };
use actix_web::cookie::{ Cookie };
use askama::Template;
use serde::{ Deserialize, Serialize };

use crate::game_logic::{ self, LetterScore };



#[derive(Deserialize)]
struct WordToCheck {
    pub guess_word: String,
}

#[derive(Deserialize)]
struct AuthCodeQuery {
    code: String,
}

/* 
 * 
 * 
 * 
 * 
 * ===================================
 * ===================================
 * =====                         =====
 * =====  ASKAMA HTML TEMPLATES  =====
 * =====                         =====
 * ===================================
 * ===================================
 * 
 * 
 * 
 * 
 */



#[derive(Template)]
#[template(path ="index.html")]
struct HomeTemplate {
    title: String,
    message: String,
}


#[derive(Template)]
#[template(path ="game.html")]
struct GameTemplate {
    title: String,
}


/* 
 * 
 * 
 * ========================
 * ========================
 * =====              =====
 * =====  GET ROUTES  =====
 * =====              =====
 * ========================
 * ========================
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 */




/* ROOT DOMAIN */
#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    //let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let home_template: HomeTemplate = HomeTemplate {
        title: "CRANKWORD".to_string(),
        message: "Welcome to Crankword!".to_string()
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* ROOT DOMAIN */
#[get("/game")]
async fn game(req: HttpRequest) -> impl Responder {
    //let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let game_template: GameTemplate = GameTemplate {
        title: "CRANKWORD".to_string()
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(game_template.render().unwrap())
 }
 

 /**
  * LOGIN RECEPTION
  * After the user logs in on auth app,
  * they are redirected here.
  */
#[get("/reception")]
async fn reception(req: HttpRequest, query: web::Query<AuthCodeQuery>) -> impl Responder {
    let auth_code: String = query.code.to_owned();

    println!("auth_code: {}", auth_code);

    // IN THIS FUNCTION we will CALL the AUTH APP and RECEIVE the REFRESH_TOKEN
    // THEN we will CREATE A JWT
    // THEN we will put BOTH into the RESPONSE
    // THEN we will create MIDDLEWARE to put those BOTH in COOKIES
    // THEN we will REDIRECT to DASHBOARD

    Redirect::to("/game")
}

 /* 
 * 
 * 
 * 
 * 
 * =========================
 * =========================
 * =====               =====
 * =====  POST ROUTES  =====
 * =====               =====
 * =========================
 * =========================
 * 
 * 
 * 
 * 
*/

#[post("/check_word")]
pub async fn check_word(
    req: HttpRequest,
    word_json: web::Json<WordToCheck>
) -> HttpResponse {
    let winning_word: String = db::get_winning_word(5).await;
    let result: Vec<LetterScore> = game_logic::check_word(&word_json.guess_word, &winning_word);

    HttpResponse::Ok().json(result)
}