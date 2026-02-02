use actix_web::{
    web, HttpResponse, Responder, HttpRequest,
    http::header, get, post, web::Redirect,
    cookie::{ Cookie }
};
use askama::Template;
use hash_ids::HashIds;
use sqlx::{ MySqlPool };
use time::OffsetDateTime;

use crate::{
    auth, auth_code_shared::{ 
        AuthCodeRequest,
        AuthCodeSuccess
    }, db::{self, GameAndPlayers, PlayerStats},
    game_logic::{ self, GameStatus },
    crankword_io, resource_mgr::{self, *}, resources::get_translation,
    routes_utils::*, utils::{ self, SupportedLangs }, words_all
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


/**
  * Redirect user to auth_app for login
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
        Err(e) => {
            eprintln!("ERROR: {}", e);
            // TODO: return a response here. Don't just put an err msg in the querystring.
            "ERROR RETRIEVING CLIENT ID".to_string()
        }
    };

    login_url.push_str(&querystring);

    HttpResponse::Found()
        .append_header(("Location", login_url))
        .finish()
}


/**
  * Redirect user to auth_app for registration
  */
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
        Err(e) => {
            eprintln!("ERROR: {}", e);
            // TODO: return a response here. Don't just put an err msg in the querystring.
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
    let texts: HomeTexts = HomeTexts::new(&user_req_data);

    let home_template: HomeTemplate = HomeTemplate {
        title: "CRANKWORD".to_string(),
        message: "Welcome to Crankword!".to_string(),
        user: user_req_data,
        texts
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* /game needs a game_id. So this just redirects to dashboard. */
#[get("/game")]
async fn game_root(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let role: &String = user_req_data.get_role();
    let redirect_location: &str = if role == "guest" { "/login" } else { "/dashboard" };

    HttpResponse::Found() // 302 redirect
        .append_header((header::LOCATION, redirect_location))
        .finish()
 }


 #[get("/game/{hashed_game_id}")]
 async fn game(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    path: web::Path<String>
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    if user_req_data.role == "guest" || user_req_data.id.is_none() {
        return redirect_to_login();
    }

    let hashed_game_id: String = match path.into_inner().parse::<String>() {
        Ok(hashid) => hashid,
        Err(_) => "400".to_string()
    };

    // The URL is hashed. Decode it.
    let game_id: i32 = match hash_ids.decode(&hashed_game_id) {
        Ok(hash_ids) => {
            if hash_ids.len() > 0 {
                hash_ids[0] as i32
            } else {
                return redirect_to_err("404")
            }
        },
        Err(_e) => return redirect_to_err("404")
    };

    let game: db::GameAndPlayers = match db::get_game_and_players(&pool, game_id).await {
        Ok(game) => game,
        Err(_e) => return redirect_to_err("404")
    };

    // Each game status option has its own page to render
    // Each option is in a function
    match game.game.game_status {
        game_logic::GameStatus::PreGame =>
            go_to_pregame(&hashed_game_id, game, user_req_data).await,
        game_logic::GameStatus::InProgress =>
            go_to_inprogress_game(&hashed_game_id, game, user_req_data).await,
        game_logic::GameStatus::Finished =>
            go_to_finished_game(game, user_req_data).await,
        game_logic::GameStatus::Cancelled =>
            go_to_cancelled_game(game, user_req_data).await
    }
}

/* FUNCTIONS TO SUPPORT THE /game/{game_id} ROUTE */

async fn go_to_pregame(
    hashed_game_id: &String,
    the_game: db::GameAndPlayers,
    user_req_data: auth::UserReqData
) -> HttpResponse {

    println!("{}", the_game.game.game_status.to_string());
    let pre_game_template: PreGameTemplate = PreGameTemplate {
        texts: resource_mgr::PreGameTexts::new(&user_req_data),
        game: the_game,
        user: user_req_data,
        hashed_game_id: hashed_game_id.to_owned()
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(pre_game_template.render().unwrap());
}


async fn go_to_inprogress_game(
    hashed_game_id: &String,
    the_game: db::GameAndPlayers,
    user_req_data: auth::UserReqData
) -> HttpResponse {
    println!("{}", the_game.game.game_status.to_string());
    let texts: GameTexts = GameTexts::new(&user_req_data);
    let game_template: GameTemplate = GameTemplate {
        title: "CRANKWORD".to_string(),
        user: user_req_data,
        game: the_game,
        texts,
        hashed_game_id: hashed_game_id.to_owned()
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(game_template.render().unwrap());
}


/**
 * NOT A ROUTE
 * This provides an HttpResponse for a user
 * requesting to visit a game which is
 * already finished/completed.
 */
async fn go_to_finished_game(
    the_game: db::GameAndPlayers,
    user_req_data: auth::UserReqData
) -> HttpResponse {
    println!("GAME STATUS: {}", the_game.game.game_status.to_string());
    let post_game_texts: PostGameTexts = resource_mgr::PostGameTexts::new(
        &user_req_data,
        None,
        false
    );

    let winner_name: Option<String> = match the_game.game.winner_id {
        None => None,
        Some(winner_id) => {
            // winner_id exists
            // get their info from the players' list
            let winner_info_option: Option<&db::PlayerInfo> =
                the_game.players
                    .iter()
                    .find(
                        |player|
                            player.user_id == winner_id
                    );
            
            // if winner info exists in players' list, get their username
            match winner_info_option {
                None => None,
                Some(winner_info) =>
                    Some(winner_info.username.to_owned())
            }
        }
    };

    let finished_game_template: FinishedGameTemplate = FinishedGameTemplate {
        texts: post_game_texts,
        user: user_req_data,
        game: the_game,
        winner_name
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(finished_game_template.render().unwrap());
}


/**
 * NOT A ROUTE
 * This provides an HttpResponse for a user
 * requesting to visit a game which
 * has been cancelled (not completed).
 */
async fn go_to_cancelled_game(
    the_game: db::GameAndPlayers,
    user_req_data: auth::UserReqData
) -> HttpResponse {
    println!("GAME STATUS: {}", the_game.game.game_status.to_string());
    let post_game_texts: PostGameTexts = resource_mgr::PostGameTexts::new(
        &user_req_data,
        None,
        true
    );

    let cancelled_game_template: CancelledGameTemplate = CancelledGameTemplate {
        texts: post_game_texts,
        user: user_req_data,
        game: the_game
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(cancelled_game_template.render().unwrap());
}



/* PLAYER DASHBOARD ROUTE */
#[get("/dashboard")]
async fn dashboard(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.role == "guest" ||
        user_req_data.id.is_none() ||
        user_req_data.username.is_none()
    { return redirect_to_login() }

    let user_id: i32 = user_req_data.id.unwrap();
    let username: String = user_req_data.username.to_owned().unwrap();

    let all_user_games: Vec<db::GameItemData> = match db::get_current_games(
        &pool, user_id).await {
        Ok(games) => games,
        Err(_e) => return redirect_to_err("500")
    };

    // Create stats object
    let mut wins: u32 = 0;
    let mut past_games: u32 = 0;
    let mut cancelled_games: u32 = 0;
    let mut current_games: Vec<db::GameLinkData> = Vec::new();

    for user_game in all_user_games {

        // rule out current games (in progress and pre-game)
        if user_game.game_status == game_logic::GameStatus::InProgress.to_string() ||
            user_game.game_status == game_logic::GameStatus::PreGame.to_string()
        {
            current_games.push(db::GameLinkData {
                hashid: hash_ids.encode(&[user_game.id as u64]),
                game_status: user_game.game_status
            });

            continue;
        }

        past_games += 1;
        
        if user_game.game_status == game_logic::GameStatus::Cancelled.to_string() {
            cancelled_games += 1;
        } else if user_game.winner_id.is_some() {
            if user_game.winner_id.unwrap() == user_id {
                wins += 1;
            }
        }
    }

    let stats: PlayerStats = PlayerStats { wins, past_games, cancelled_games };
    let raw_invitations: Vec<db::GameId> = match db::get_invitations_by_username(&pool, username).await {
        Ok(invites) => invites,
        Err(_e) => return redirect_to_err("500")
    };

    // hash each id into a new vector
    let invited_game_hashes: Vec<String> = raw_invitations
        .iter()
        .map(|raw_invite| hash_ids.encode(&[raw_invite.game_id as u64]))
        .collect();

    let dash_template: DashboardTemplate = DashboardTemplate {
        texts: DashTexts::new(&user_req_data),
        user: user_req_data,
        current_games,
        stats,
        invited_game_hashes
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
        crankword_io::check_auth_code(client_auth_data).await;

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


/**
 * During an in-progress game, get a list of the current players,
 * We will NOT sort players here. We'll offload that onto the client.
 * Instead we will just send a list of players, and the current_player_id.
 * the current_player_id is what will change most often.
 * 
 * Each player object will also have a list of scores from their guesses,
 * so the user can see how the opponents are doing.
 */
#[post("refresh_in_prog_players")]
pub async fn refresh_in_prog_players(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    hashed_game_id: web::Json<HashedGameId>
) -> HttpResponse {
    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let player_id: i32 = match user_req_data.id {
        Some(id) => id,
        None => return return_unauthorized_err_json(&user_req_data)
    };

    let game_id: i32 = match hash_ids.decode(&hashed_game_id.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 {
                ids[0] as i32
            } else {
                return return_internal_err_json()
            }
        },
        Err(_e) => return return_internal_err_json()
    };

    // get the game
    let mut the_game: db::Game = match db::get_game_by_id(&pool, game_id).await {
        Ok(g) => g,
        Err(_) => return return_unauthorized_err_json(&user_req_data)
    };


    // Get the players with their scores, but no words in the scores
    let players: Vec<db::PlayerRefreshData> =
        match db::get_players_refresh_data_by_game_id(&pool, &the_game).await {
            Ok(p) => p,
            Err(_) => return return_unauthorized_err_json(&user_req_data)
        };
    

    // CHECK FOR TIMEOUT AND SWITCH TURN
    // Only GAME OWNER checks and initiates switch_turn
    // and only for multi-player games
    if the_game.owner_id == player_id && players.len() > 1 {
        let now: OffsetDateTime = OffsetDateTime::now_utc();
        if now >= the_game.turn_timeout && the_game.turn_user_id.is_some() {
            println!("TIMEOUT: FORCE CHANGE TURN!");
            let current_turn_user_id: i32 = the_game.turn_user_id.unwrap();

            // PUT DUDS into player who missed a turn
            // get the NUMBER of player guesses.
            let turn_player_guess_count: u8 =
                match db::get_guess_count(
                    &pool,
                    game_id,
                    current_turn_user_id
                ).await {
                    Ok(count) => count,
                    Err(_e) => return return_internal_err_json()
                };
            
            // Insert dud guess
            let dud_word: &str = "-----";
            let _insert_dud_result: i64 = match db::new_guess(
                &pool,
                current_turn_user_id,
                game_id,
                dud_word,
                turn_player_guess_count + 1
            ).await {
                Ok(new_id) => new_id,
                Err(_) => return return_internal_err_json()
            };

            // Actually switch the turn
            let _next_turn_result: i32 = match db::next_turn(&pool, game_id).await {
                Ok(new_user_turn_id) => new_user_turn_id,
                Err(_) => return return_unauthorized_err_json(&user_req_data)
            };

            // Turn has been switched. Refresh game object.
            the_game = match db::get_game_by_id(&pool, game_id).await {
                Ok(g) => g,
                Err(_) => return return_unauthorized_err_json(&user_req_data)
            };

            // check if game is over
            // 1. check if this was player's final turn
            // 2. if so, check if anybody else has remaining turns
            // 3. if nobody else can play, game over (no winner)

            if turn_player_guess_count + 1 >= game_logic::MAX_TURNS {
                /*
                * This was the final turn, and NOT the correct guess.
                * So it's game over for this player.
                * So check if anybody else still has a turn.
                */

                let turns_still_exist_result: Result<bool, anyhow::Error> =
                    db::somebody_can_play(&pool, game_id).await;

                if turns_still_exist_result.is_err() {
                    return return_internal_err_json();
                }
                
                let turns_still_exist: bool = turns_still_exist_result.unwrap();

                if !turns_still_exist {
                    // game is over.
                    let _finish_game_result: Result<u8, anyhow::Error> =
                        finish_game(&pool, game_id, None).await;
                }
            }
        }
    }


    // Client must know whose turn it is
    let current_turn_id: i32 = match the_game.turn_user_id {
        Some(id) => id,
        None => return return_unauthorized_err_json(&user_req_data)
    };

    let game_over: bool = the_game.game_status != GameStatus::InProgress;
    let turn_timeout: time::OffsetDateTime = the_game.turn_timeout;

    let in_prog_refresh: InProgRefresh = InProgRefresh {
        current_turn_id,
        players,
        game_over,
        turn_timeout,
    };

    // only send the data if the user really belongs to this game
    match in_prog_refresh.user_id_is_player(player_id) {
        true => HttpResponse::Ok().json(in_prog_refresh),
        false => return_unauthorized_err_json(&user_req_data)
    }    
}



/**
 * Get fresh data about game status and who the players are.
 */
#[post("/refresh_pregame")]
pub async fn refresh_pregame(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    hashed_game_id: web::Json<HashedGameId>
) -> HttpResponse {
    println!("REFRESHING GAME");
    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    if user_req_data.get_role() == "guest" {
        return return_unauthorized_err_json(&user_req_data);
    }

    let game_id: i32 = match hash_ids.decode(&hashed_game_id.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 {
                ids[0] as i32
            } else {
                return return_internal_err_json()
            }
        },
        Err(_e) => return return_internal_err_json()
    };

    let the_game: db::GameAndPlayers = match db::get_game_and_players(&pool, game_id).await {
        Ok(gap) => gap,
        Err(_e) => return return_internal_err_json()
    };

    let invitee_usernames: Vec<String> =
        match db::get_invitee_usernames(&pool, game_id).await {
            Ok(usernames) => usernames,
            Err(_e) => return return_internal_err_json()
        };

    let refresh_data: PreGameRefresh = PreGameRefresh {
        game_status: the_game.game.game_status,
        players: the_game.players,
        invitee_usernames
    };

    HttpResponse::Ok().json(refresh_data)
}



#[post("/join_game")]
pub async fn join_game(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    game_join_hash_id: web::Json<HashedGameId>
) -> HttpResponse {
    println!("JOINING GAME");
    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    if user_req_data.get_role() == "guest" {
        return return_unauthorized_err_json(&user_req_data);
    }

    let game_id: i32 = match hash_ids.decode(&game_join_hash_id.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 { ids[0] as i32 }
            else { return return_internal_err_json() }
        },
        Err(_e) => return return_internal_err_json()
    };

    // Make sure they're not already in too many pregame or inprogress games.
    let games_count: u8 = 
        match db::get_current_games_count(&pool, user_req_data.id.unwrap()).await {
            Ok(count) => count,
            Err(_e) => return return_internal_err_json()
        };

    if games_count >= utils::MAX_CURRENT_GAMES {
        return HttpResponse::Ok().json(JoinGameFailure {
            success: false,
            error: "You're in too many current games".to_string()
        });
    }

    let other_players_count: u8 =
        match db:: get_game_players_count(&pool, game_id).await {
            Ok(count) => count,
            Err(_e) => return return_internal_err_json()
        };

    if other_players_count >= utils::MAX_PLAYERS {
        return HttpResponse::Ok().json(JoinGameFailure {
            success: false,
            error: "Too many current players".to_string()
        });
    }

    // Use may join
    let user_joined_game: bool = match db::user_join_game(
        &pool,
        &user_req_data,
        game_id
    ).await {
        Ok(joined) => joined,
        Err(e) => {
            return HttpResponse::Ok().json(JoinGameFailure {
                error: e.to_string(),
                success: false
            });
        }
    };

    if user_joined_game {
        // delete invitation. Don't worry about the result.
        let _delete_result: Result<u8, anyhow::Error> =
            db::delete_invite(
                &pool,
                game_id,
                &user_req_data.get_username()
            ).await;
    }

    HttpResponse::Ok().json(JoinGameSuccess { success: user_joined_game })
}


#[post("/start_game")]
pub async fn start_game(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    game_start_id: web::Json<HashedGameId>
) -> HttpResponse {
    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    if user_req_data.get_role() == "guest" {
        return return_unauthorized_err_json(&user_req_data);
    }

    let game_id: i32 = match hash_ids.decode(&game_start_id.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 {
                ids[0] as i32
            } else {
                return return_internal_err_json()
            }
        },
        Err(_e) => return return_internal_err_json()
    };

    let the_game: db::Game = match db::get_game_by_id(&pool, game_id).await {
        Ok(the_game) => the_game,
        Err(_e) => return return_internal_err_json()
    };

    if the_game.owner_id != user_req_data.id.unwrap() {
        // TODO: put the error message into the resources file
        return HttpResponse::Ok().json(StartGameFailure {
            error: "Only the game owner can start a game.".to_string(),
            success: false
        });
    } else if the_game.game_status != GameStatus::PreGame {
        // TODO: put the error message into the resources file
        return HttpResponse::Ok().json(StartGameFailure {
            error: "Game has already started.".to_string(),
            success: false
        });
    }

    // Call the db to change the status of the game
    let game_started: StartGameSuccess =
        match db::start_game(&pool, the_game.id).await {
            Ok(success) => StartGameSuccess {success},
            Err(_e) => return return_internal_err_json()
        };


    // delete all invitations (some may be pending, so still extant)
    if game_started.success {
        let _deleted_invite_count_result: Result<u8, anyhow::Error> =
            db::delete_invites(&pool, game_id).await;
    }

    HttpResponse::Ok().json(game_started)
}


#[post("/new_game")]
pub async fn new_game(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest
) -> HttpResponse {
    // make sure it's a real user
    // make the game and get the id
    // redirect user to game page
    // on game page show word (FOR NOW.... obviously later we will NOT show that)

    // Make sure it's a real user
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    if user_req_data.get_role() == "guest" {
        return return_unauthorized_err_json(&user_req_data);
    }

    // make the game and get the id
    let user_id: i32 = user_req_data.id.unwrap();

    // Make sure they're not already in a pregame or inprogress game.
    let games_count: u8 = 
        match db::get_current_games_count(&pool, user_id).await {
            Ok(count) => count,
            Err(_e) => return return_internal_err_json()
        };

    if games_count >= utils::MAX_CURRENT_GAMES {
        return HttpResponse::Ok().json(JoinGameFailure {
            success: false,
            error: "Too many current games".to_string()
        });
    }

    let game_id: i32 = match db::new_game(&pool, &user_req_data).await {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(
                ErrorResponse {
                error: e.to_string(),
                code: 404
    })}};

    // send back the game_id so the front-end can redirect.
    println!("created game object: {}", game_id);
    HttpResponse::Ok().json(HashedGameId { 
        hashed_game_id: hash_ids.encode(&[game_id as u64])
     })
}

/**
 * One of the most important functions.
 * User's guesses must be checked in multiple ways:
 * 1. get the GAME -- NEW STRUCT which includes PLAYER IDs
 * 2. make sure user belongs in game AND it is user's turn
 * 3. make sure word is REAL WORD
 * 4. add guess to DB table
 * 5. check word against winning word and return vector of LetterScores
 */
#[post("/check_guess")]
pub async fn check_guess(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    word_json: web::Json<WordToCheck>
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let user_id: i32 = match user_req_data.id {
        Some(id) => id,
        None => {
            // user is guest
            let error: String = get_translation("err.403.body", &user_req_data.lang, None);
            return HttpResponse::Unauthorized().json(
                ErrorResponse {
                error,
                code: 403
            });
    }};

    let game_id: i32 = match hash_ids.decode(&word_json.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 {
                ids[0] as i32
            } else {
                return return_internal_err_json()
            }
        },
        Err(_e) => return return_internal_err_json()
    };

    // User is logged in
    // Get the game
    let game_and_players: GameAndPlayers =
        match db::get_game_and_players(&pool, game_id).await {
            Ok(data) => data,
            Err(_e) => return return_internal_err_json()
        };

    // Make sure user is a player
    if !game_and_players.user_is_player(
        db::PlayerInfo {
            user_id,
            username: user_req_data.get_username()
        }
    ) {
        // User is NOT player for this game.
        let error: String = get_translation("err.403.body", &user_req_data.lang, None);
        return HttpResponse::Unauthorized().json(
            ErrorResponse {
            error,
            code: 403
        });
    }

    // Make sure it is player's turn
    // If it's not the user's turn, return a json object which indicates that.
    let player_turn: bool = game_and_players.game.turn_user_id.is_some() &&
        game_and_players.game.turn_user_id.unwrap() == user_id;

    if !player_turn {
        return HttpResponse::Ok().json(WrongTurn::new());
    }

    // get the NUMBER of player guesses.
    let player_guess_count: u8 =
        match db::get_guess_count(&pool, game_id, user_id).await {
            Ok(count) => count,
            Err(_e) => return return_internal_err_json()
        };

    if player_guess_count >= game_logic::MAX_TURNS {
        return HttpResponse::Ok().json(MaxGuesses::new());
    }

    // make sure guess word is REAL WORD
    if !words_all::is_real_word(&word_json.guess_word) {
        return HttpResponse::Ok().json(FakeWord::new());
    }

    // add guess to the DB
    let add_guess_result: Result<i64, anyhow::Error> = db::new_guess(
        &pool,
        user_id,
        game_and_players.game.id,
        &word_json.guess_word,
        player_guess_count + 1
    ).await;

    if add_guess_result.is_err() {
        eprintln!("Error adding guess result");
        return return_internal_err_json();
    }
    
    // TODO: ADD AUTH CHECKS (user belongs to game, it is user's turn).
    // If it's not the user's turn, return a json object which indicates that.
    let winning_word: String = match db::get_winning_word(&pool, game_id).await {
        Ok(word) => word,
        Err(_e) => {
            return HttpResponse::Unauthorized().json(ErrorResponse{
                error: String::from("Not Found"),
                code: 406
            })
        }
    };

    let guess_result_basic: game_logic::CheckGuessResultBasic =
        game_logic::check_guess(&word_json.guess_word, &winning_word);
    
    let mut guess_result: game_logic::CheckGuessResult = 
        game_logic::CheckGuessResult::new(
            guess_result_basic,
            false,
            user_id
        );

    // Do we have a winner?
    if guess_result.is_winner {
        let finish_game_result: Result<u8, anyhow::Error> =
            finish_game(&pool, game_id, Some(user_id)).await;
        
        if finish_game_result.is_err() {
            return return_internal_err_json();
        }

        guess_result.game_over = true;

        println!("GAME OVER WINNER");

    } else {

        // make it the next player's turn:
        let next_turn_id: i32 =
            match db::next_turn(&pool, game_id).await {
                Ok(new_id) => new_id,
                Err(_) => {
                    eprintln!("Error switching turns.");
                    return return_internal_err_json();
                }
            };

        // Make sure the user knows whose turn is next.
        guess_result.next_turn_id = next_turn_id;

        // check if game is over
        // 1. check if this was player's final turn
        // 2. if so, check if anybody else has remaining turns
        // 3. if nobody else can play, game over (no winner)

        if player_guess_count + 1 >= game_logic::MAX_TURNS {
            /*
             * This was the final turn, and NOT the correct guess.
             * So it's game over for this player.
             * So check if anybody else still has a turn.
             */

            let turns_still_exist_result: Result<bool, anyhow::Error> =
                db::somebody_can_play(&pool, game_id).await;

            if turns_still_exist_result.is_err() {
                return return_internal_err_json();
            }
            
            let turns_still_exist: bool = turns_still_exist_result.unwrap();

            if !turns_still_exist {
                // game is over.
                let _finish_game_result: Result<u8, anyhow::Error> =
                    finish_game(&pool, game_id, None).await;
                guess_result.game_over = true;
            }
        }
    }

    HttpResponse::Ok().json(guess_result)
}


/**
 * TODO:
 * All errors should send back THE SAME OBJECT.
 * inviet_success can be FALSE
 * and include a MESSAGE explaining what went wrong.
 */
#[post("/invite_player")]
pub async fn invite_player(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    invite_data: web::Json<InviteData>
 ) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let user_id: i32 = match user_req_data.id {
        Some(id) => id,
        None => { return return_unauthorized_err_json(&user_req_data); }
    };

    // The game id is hashed. Decode it.
    let game_id: i32 = match hash_ids.decode(&invite_data.hashed_game_id) {
        Ok(hash_ids) => {
            if hash_ids.len() > 0 {
                hash_ids[0] as i32
            } else {
                return redirect_to_err("404")
            }
        },
        Err(_e) => return redirect_to_err("404")
    };

    // get game and make sure user is owner
    let the_game: db::Game = match db::get_game_by_id(&pool, game_id).await {
        Ok(g) => g,
        Err(_e) => return redirect_to_err("404")
    };

    if the_game.owner_id != user_id {
        return redirect_to_err("403");
    }

    // make sure we don't already have too many invites
    let invites_count: u8 =
        match db:: get_invites_count(&pool, game_id).await {
            Ok(count) => count,
            Err(_e) => return return_internal_err_json()
        };

    if invites_count >= utils::MAX_INVITES {
        let success_object: InviteSuccessObject = InviteSuccessObject {
            invite_success: false,
            message: "Max invites reached".to_string()
        };
        
        return HttpResponse::Ok().json(success_object);
    }

    // user is owner. Make the invite
    let invite_success: bool =
        match db::invite_user(
            &pool,
            &invite_data.invited_player_username,
            game_id
        ).await {
            Ok(invited) => invited,
            Err(_e) => return redirect_to_err("404")
        };

    let message: String = if invite_success {
        "User invited".to_string()
    } else {
        "User not invited".to_string()
    };

    let success_object: InviteSuccessObject = InviteSuccessObject {
        invite_success,
        message
    };

    HttpResponse::Ok().json(success_object)
 }



/**
 * Returns a vec of vecs of LetterScore structs.
 * This is in case we need an update after the page is loaded.
 */
#[post("/get_guess_scores")]
pub async fn get_guess_scores(
    pool: web::Data<MySqlPool>,
    hash_ids: web::Data<HashIds>,
    req: HttpRequest,
    hashed_game_id: web::Json<HashedGameId>
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let user_id: i32 = match user_req_data.id {
        Some(id) => id,
        None => { return return_unauthorized_err_json(&user_req_data); }
    };

    let game_id: i32 = match hash_ids.decode(&hashed_game_id.hashed_game_id) {
        Ok(ids) => {
            if ids.len() > 0 {
                ids[0] as i32
            } else {
                return return_internal_err_json()
            }
        },
        Err(_e) => return return_internal_err_json()
    };

    let all_scores: Vec<game_logic::GuessAndScore> =
        match db::get_guess_scores(&pool, game_id, user_id).await {
            Ok(scores) => scores,
            Err(_e) => return return_unauthorized_err_json(&user_req_data)
        };
    
    println!("GUESSES: {}", all_scores.len());

    // now I have all the scores. I need to serialize them and deliver them.
    let scores_obj: AllPlayerScores = AllPlayerScores {
        scores: all_scores
    };

    HttpResponse::Ok().json(scores_obj)
}