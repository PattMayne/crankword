use actix_web::{
    web, HttpResponse, Responder, HttpRequest,
    http::header, get, post, web::Redirect,
    cookie::{ Cookie }
};
use askama::Template;
use serde::{ Deserialize, Serialize };

use crate::{
    auth, auth_code_shared::{ 
        AuthCodeRequest,
        AuthCodeSuccess
    }, db, game_logic::{ self, LetterScore }, io, resource_mgr::*,
    resources::get_translation, utils::SupportedLangs, words_solutions,
    words_all,
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
    pub game_id: i32,
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


#[derive(Serialize)]
pub struct GameId {
    pub game_id: i32,
}

#[derive(Serialize)]
pub struct FakeWord {
    pub fake_word: bool,
}


impl FakeWord {
    pub fn new() -> FakeWord {
        FakeWord {
            fake_word: true
        }
    }
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
    user: auth::UserReqData,
}


#[derive(Template)]
#[template(path ="game.html")]
struct GameTemplate {
    title: String,
    user: auth::UserReqData,
}


#[derive(Template)]
#[template(path="dashboard.html")]
struct DashboardTemplate {
    texts: DashTexts,
    user: auth::UserReqData,
    current_games: Vec<db::GameItemData>,
}


#[derive(Template)]
#[template(path ="error.html")]
struct ErrorTemplate {
    error_data: ErrorData,
    user: auth::UserReqData,
    texts: ErrorTexts,
}



// OTHER STRUCTS & ENUMS


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


 #[get("/login")]
 async fn login() -> HttpResponse {
    let lang: SupportedLangs = SupportedLangs::English;
    let mut login_url: String = get_translation(
        "links.login",
        &lang,
        None
    );

    let querystring: String = match std::env::var("CLIENT_ID") {
        Ok(client_id) => {
            "?client_id=".to_string() + &client_id
        },
        Err(_e) => {
            eprintln!("");
            "ERROR RETRIEVING CLIENT ID".to_string()
        }
    };

    login_url.push_str(&querystring);

    HttpResponse::Found()
        .append_header(("Location", login_url))
        .finish()
 }


 #[get("/register")]
 async fn register() -> HttpResponse {
    let lang: SupportedLangs = SupportedLangs::English;
    let mut register_url: String = get_translation(
        "links.register",
        &lang,
        None
    );

    let querystring: String = match std::env::var("CLIENT_ID") {
        Ok(client_id) => {
            "?client_id=".to_string() + &client_id
        },
        Err(_e) => {
            eprintln!("");
            "ERROR RETRIEVING CLIENT ID".to_string()
        }
    };

    register_url.push_str(&querystring);

    HttpResponse::Found()
        .append_header(("Location", register_url))
        .finish()
 }



#[get("/logout")]
pub async fn logout() -> HttpResponse {

    let jwt_cookie: Cookie<'_> = Cookie::build("jwt", "")
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .finish();

    let refresh_cookie: Cookie<'_> = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .finish();

    // TO DO: call auth_app to delete refresh_token from DB
    
    HttpResponse::Found() // 302 redirect
        .cookie(jwt_cookie)
        .cookie(refresh_cookie)
        .append_header((header::LOCATION, "/"))
        .finish()
}




/* ROOT DOMAIN */
#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let home_template: HomeTemplate = HomeTemplate {
        title: "CRANKWORD".to_string(),
        message: "Welcome to Crankword!".to_string(),
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* ROOT DOMAIN */
#[get("/game")]
async fn game_root(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let role: &String = user_req_data.get_role();
    let redirect_location: &str = if role == "guest" { "/login" } else { "/dashboard" };

    HttpResponse::Found() // 302 redirect
        .append_header((header::LOCATION, redirect_location))
        .finish()
 }

 #[get("/game/{game_id}")]
 async fn game(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.role == "guest" {
        return redirect_to_login();
    }

    let game_id: i32 = match path.into_inner().parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return redirect_to_err("400");
        }
    };

    println!("game id: {}", game_id);

    let game_template: GameTemplate = GameTemplate {
        title: "CRANKWORD".to_string(),
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(game_template.render().unwrap())
 }


/* PLAYER DASHBOARD */
#[get("/dashboard")]
async fn dashboard(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.role == "guest" {
        return redirect_to_login();
    }

    let dash_template: DashboardTemplate = DashboardTemplate {
        texts: DashTexts::new(&user_req_data),
        user: user_req_data,
        current_games: Vec::new(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(dash_template.render().unwrap())
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

    // IN THIS FUNCTION we will CALL the AUTH APP and RECEIVE the REFRESH_TOKEN

    let client_id: String = match std::env::var("CLIENT_ID") {
        Ok(secret) => secret,
        Err(_e) => {
            eprintln!("ERROR: NO CLIENT ID.");
            return redirect_to_err("404");
        }
    };

    let client_secret: String = match std::env::var("CLIENT_SECRET") {
        Ok(secret) => secret,
        Err(_e) => {
            eprintln!("ERROR: NO CLIENT SECRET.");
            return redirect_to_err("400");
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
                    eprintln!("Error: {}", e);
                    return redirect_to_err("404");
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
            return redirect_to_err("404");
        }
    }
}


// Function for the catch-all "not found" route
pub async fn not_found() -> impl Responder {
    Redirect::to("/error/404")
}


#[get("/error/{code}")]
async fn error_page(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let code: String = match path.into_inner().parse::<String>() {
        Ok(code) => code,
        Err(_) => "400".to_string()
    };

    let error_data: ErrorData = ErrorData::new(
        code,
        &user_req_data.lang
    );

    let error_template: ErrorTemplate<> = ErrorTemplate {
        error_data,
        texts: ErrorTexts::new(&user_req_data),
        user: user_req_data
    };

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


// if user just goes to /auth
pub fn redirect_to_err(err_code: &str) -> HttpResponse {
    let new_location: String = format!("/error/{}", err_code);
    HttpResponse::Found()
        .append_header(("Location", new_location))
        .finish()
}


// redirect user to login page
pub fn redirect_to_login() -> HttpResponse {
    HttpResponse::Found() // 302 redirect
        .append_header((header::LOCATION, "/login"))
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

#[post("/new_game")]
pub async fn new_game(req: HttpRequest) -> HttpResponse {
    // make sure it's a real user
    // make the game and get the id
    // redirect user to game page
    // on game page show word (FOR NOW.... obviously later we will NOT show that)

    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.get_role() == "guest" {
        let error: String = get_translation("err.empty_creds", &user_req_data.lang, None);
        return HttpResponse::Unauthorized().json(
            ErrorResponse {
            error,
            code: 401
        });
    }

    // make the game and get the id
    let user_id: i32 = user_req_data.id.unwrap();
    let game_id: i32 = match db::new_game(user_id).await {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(
                ErrorResponse {
                error: e.to_string(),
                code: 404
            });
        }
    };

    // redirect user to game page

    // NO... we must send back the game_id so the JS can redirect.

    println!("created game object: {}", game_id);

    HttpResponse::Ok().json(GameId { game_id })
}

#[post("/check_guess")]
pub async fn check_guess(
    req: HttpRequest,
    word_json: web::Json<WordToCheck>
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.get_role() == "guest" {
        let error: String = get_translation("err.empty_creds", &user_req_data.lang, None);
        return HttpResponse::Unauthorized().json(
            ErrorResponse {
            error,
            code: 401
        });
    }

    // User is logged in
    // make sure guess is part of the ACCEPTED actual words
    if !words_all::check_word(&word_json.guess_word) {
        println!("NOT A REAL WORD");
        return HttpResponse::Ok().json(FakeWord::new());
    }

    // save the GUESS to the GUESS TABLE
    // IF SO... get the winning_word from the actual game

    // GET THE GAME. Don't just get the word. Get the GAME and see whose TURN it is.
    // 1. get the GAME -- NEW STRUCT which includes PLAYER IDs
    // 2. make sure user belongs in game AND it is user's turn
    // 3. make sure word is REAL WORD
    // 4. add guess to DB table
    // 5. check word against winning word and return vector of LetterScores

    // TODO: ADD AUTH CHECKS (user belongs to game, it is user's turn)
    let winning_word: String = match db::get_winning_word(word_json.game_id).await {
        Ok(word) => word,
        Err(_e) => {
            return HttpResponse::Unauthorized().json(ErrorResponse{
                error: String::from("Not Found"),
                code: 406
            })
        }
    };

    println!("Winning word is: {}", winning_word);
    println!("Guess word is: {}", word_json.guess_word);

    let guess_result: Vec<LetterScore> =
        game_logic::check_guess(&word_json.guess_word, &winning_word);
    HttpResponse::Ok().json(guess_result)
}