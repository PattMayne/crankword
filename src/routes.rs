use actix_web::{
    web, HttpResponse, Responder, HttpRequest,
    http::header, get, post, web::Redirect,
    cookie::{ Cookie }
};
use askama::Template;
use serde::{ Deserialize, Serialize };

use crate::{
    db, auth, io,
    game_logic::{ self, LetterScore },
    resource_mgr::*,
    auth_code_shared::{ 
        AuthCodeRequest,
        AuthCodeSuccess
    }
};

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



#[derive(Deserialize)]
struct WordToCheck {
    pub guess_word: String,
}

#[derive(Deserialize)]
struct AuthCodeQuery {
    code: String,
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
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


#[derive(Template)]
#[template(path ="error.html")]
struct ErrorTemplate {
    error_data: ErrorData,
    user: auth::UserReqData,
    texts: ErrorTexts,
}



// OTHER STRUCTS


struct TwoAuthCookies {
    pub jwt_cookie: Cookie<'static>,
    pub refresh_token_cookie: Cookie<'static>,
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
async fn home() -> impl Responder {
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
async fn game() -> impl Responder {
    //let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let game_template: GameTemplate = GameTemplate {
        title: "CRANKWORD".to_string()
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(game_template.render().unwrap())
 }
 

fn redirect_to_game() -> HttpResponse {
    HttpResponse::Found() // 302 redirect
        .append_header((header::LOCATION, "/game"))
        .finish()
}


 /**
  * LOGIN RECEPTION
  * After the user logs in on auth app,
  * they are redirected here.
  */
#[get("/reception")]
async fn reception(query: web::Query<AuthCodeQuery>) -> HttpResponse {
    let auth_code: String = query.code.to_owned();

    println!("auth_code: {}", auth_code);

    // IN THIS FUNCTION we will CALL the AUTH APP and RECEIVE the REFRESH_TOKEN

    let client_id: String = match std::env::var("CLIENT_ID") {
        Ok(secret) => secret,
        Err(_e) => {
            eprintln!("ERROR: NO CLIENT ID. MAKE ERROR PAGE!");
            return redirect_to_game();
        }
    };

    let client_secret: String = match std::env::var("CLIENT_SECRET") {
        Ok(secret) => secret,
        Err(_e) => {
            eprintln!("ERROR: NO CLIENT SECRET. MAKE ERROR PAGE!");
            return redirect_to_game()
        }
    };

    let client_auth_data: AuthCodeRequest = AuthCodeRequest {
        client_id,
        client_secret,
        code: auth_code,
    };

    let auth_code_response: Result<AuthCodeSuccess, anyhow::Error> = 
        io::check_auth_code(client_auth_data).await;

    match auth_code_response {
        Ok(success) => {
            println!("Token: {}", success.refresh_token);
            println!("Name: {}", success.username);
            println!("Id: {}", success.user_id);

            // Generate a token String
            let jwt: String = match auth::generate_jwt(
                success.user_id,
                success.username,
                success.user_role
            ) {
                Ok(token) => token,
                Err(e) => {
                    println!("Error: {}", e);
                    return redirect_to_game();
                }
            };

            // Now make the cookies and set them in the response
            let jwt_cookie: Cookie<'_> = auth::build_token_cookie(
                jwt,
                String::from("jwt"));
            
            let refresh_token_cookie: Cookie<'_> = auth::build_token_cookie(
                success.refresh_token,
                String::from("refresh_token"));

            HttpResponse::Found() // 302 redirect
                .append_header((header::LOCATION, "/game"))
                .cookie(jwt_cookie)
                .cookie(refresh_token_cookie)
                .finish()

        },
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::Found() // 302 redirect
                .append_header((header::LOCATION, "/game"))
                .finish()
        }
    }


}


// Function for the catch-all "not found" route
pub async fn not_found() -> impl Responder {

    println!("here ---111");
    Redirect::to("/error/404")
}


#[get("/error/{code}")]
async fn error_page(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    println!("here 000");
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    println!("here 111");

    let code: String = match path.into_inner().parse::<String>() {
        Ok(code) => code,
        Err(_) => "400".to_string()
    };

    let error_data: ErrorData = ErrorData::new(
        code,
        &user_req_data.lang
    );

    println!("here 222");
    let error_template: ErrorTemplate<> = ErrorTemplate {
        error_data,
        texts: ErrorTexts::new(&user_req_data),
        user: user_req_data
    };
    println!("here 333");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(error_template.render().unwrap())
}


#[get("/error")]
async fn error_root() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/error/500"))
        .finish()
}


#[get("/error/")]
async fn error_root_2() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/error"))
        .finish()
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
    word_json: web::Json<WordToCheck>
) -> HttpResponse {
    let winning_word: String = db::get_winning_word(5).await;
    let result: Vec<LetterScore> = game_logic::check_word(&word_json.guess_word, &winning_word);

    HttpResponse::Ok().json(result)
}




