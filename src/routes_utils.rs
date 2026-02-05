use serde::{ Deserialize, Serialize };
use askama::Template;
use actix_web::{
    web,
    cookie::{ Cookie },
    HttpResponse,
    http::StatusCode
};
use anyhow::{ Result, };
use sqlx::{ MySqlPool };
use time::{ OffsetDateTime };

use crate::{
    game_logic::{ self,GameStatus },
    db::{ self, PlayerInfo,PlayerRefreshData },
    auth, resource_mgr::{*},
    resources::get_translation
};
use hash_ids::HashIds;


/* 
 *
 * 
 * 
 * 
 * RRRRRRRRRRRRRRRRRRRRRRRRRR
 * RRRRRRRRRRRRRRRRRRRRRRRRRR
 * RRRRR                RRRRR
 * RRRRR  ROUTES UTILS  RRRRR
 * RRRRR                RRRRR
 * RRRRRRRRRRRRRRRRRRRRRRRRRR
 * RRRRRRRRRRRRRRRRRRRRRRRRRR
 * 
 * structs, templates, and functions
 * specifically for the routes.rs module.
 * 
 * 
 * 
 * 
 * 
 * =====================
 * =====================
 * =====           =====
 * =====  STRUCTS  =====
 * =====           =====
 * =====================
 * =====================
 * 
 * 
 * 
 * 
*/


#[derive(Deserialize)]
pub struct WordToCheck {
    pub guess_word: String,
    pub hashed_game_id: String,
}

#[derive(Deserialize)]
pub struct AuthCodeQuery {
    pub code: String,
}

#[derive(Deserialize)]
pub struct DeleteInviteData {
    pub username: String,
    pub hashed_game_id: String,
}


#[derive(Deserialize)]
pub struct BootPlayerData {
    pub username: String,
    pub hashed_game_id: String,
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}

#[derive(Serialize)]
pub struct GameCancelled {
    pub success: bool,
    pub message: String,
}



#[derive(Serialize)]
pub struct AllPlayerScores {
    pub scores: Vec<game_logic::GuessAndScore>,
}


#[derive(Serialize)]
pub struct PreGameRefresh {
    pub game_status: GameStatus,
    pub players: Vec<PlayerInfo>,
    pub invitee_usernames: Vec<String>,
}

#[derive(Serialize)]
pub struct InviteSuccessObject {
    pub invite_success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct BootPlayerSuccessObject {
    pub success: bool,
    pub message: String,
}


#[derive(Serialize)]
pub struct UninviteSuccessObject {
    pub success: bool,
    pub message: String,
}



/**
 * Specifyingtime::serde::rfc3339 to make the serialized
 * timestamp JavaScript-readable.
 */
#[derive(Serialize)]
pub struct InProgRefresh {
    pub current_turn_id: i32,
    pub players: Vec<PlayerRefreshData>,
    pub game_status: GameStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub turn_timeout: OffsetDateTime,
}

#[derive(Serialize)]
pub struct DashboardRefreshData {
    pub invited_game_hashes: Vec<String>,
}

#[derive(Serialize)]
pub struct JoinGameFailure {
    pub error: String,
    pub success: bool,
}


impl JoinGameFailure {
    pub fn new(message: String) -> JoinGameFailure {
        JoinGameFailure {
            success: false,
            error: message
        }
    }
}

#[derive(Serialize)]
pub struct LeaveGameFailure {
    pub error: String,
    pub success: bool,
}


#[derive(Serialize)]
pub struct StartGameFailure {
    pub error: String,
    pub success: bool,
}


#[derive(Serialize)]
pub struct JoinGameSuccess {
    pub success: bool,
}


#[derive(Serialize)]
pub struct LeaveGameSuccess {
    pub success: bool,
}



#[derive(Serialize)]
pub struct StartGameSuccess {
    pub success: bool,
}



#[derive(Serialize, Deserialize)]
pub struct GameId {
    pub game_id: i32,
}


#[derive(Serialize, Deserialize)]
pub struct HashedGameId {
    pub hashed_game_id: String,
}



#[derive(Serialize, Deserialize)]
pub struct InviteData {
    pub hashed_game_id: String,
    pub invited_player_username: String,
}


#[derive(Serialize)]
pub struct FakeWord {
    pub fake_word: bool,
}

#[derive(Serialize)]
pub struct MaxGuesses {
    pub max_guesses: bool,
}

#[derive(Serialize)]
pub struct WrongTurn {
    pub wrong_turn: bool,
}

impl FakeWord {
    pub fn new() -> FakeWord {
        FakeWord {
            fake_word: true
        }
    }
}

impl MaxGuesses {
    pub fn new() -> MaxGuesses {
        MaxGuesses {
            max_guesses: true
        }
    }
}

impl WrongTurn {
    pub fn new() -> WrongTurn {
        WrongTurn {
            wrong_turn: true
        }
    }
}

impl InProgRefresh {

    pub fn user_id_is_player(&self, player_id: i32) -> bool {
        for player_info in &self.players {
            if player_info.user_id == player_id {
                return true;
            }
        }

        false
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
pub struct HomeTemplate {
    pub title: String,
    pub message: String,
    pub user: auth::UserReqData,
    pub texts: HomeTexts
}


#[derive(Template)]
#[template(path ="game.html")]
pub struct GameTemplate {
    pub title: String,
    pub user: auth::UserReqData,
    pub game: db::GameAndPlayers,
    pub texts: GameTexts,
    pub hashed_game_id: String,
}


#[derive(Template)]
#[template(path="pre_game.html")]
pub struct PreGameTemplate {
    pub texts: PreGameTexts,
    pub user: auth::UserReqData,
    pub game: db::GameAndPlayers,
    pub hashed_game_id: String,
}

#[derive(Template)]
#[template(path="cancelled_game.html")]
pub struct CancelledGameTemplate {
    pub texts: PostGameTexts,
    pub user: auth::UserReqData,
    pub game: db::GameAndPlayers,
}

#[derive(Template)]
#[template(path="finished_game.html")]
pub struct FinishedGameTemplate {
    pub texts: PostGameTexts,
    pub user: auth::UserReqData,
    pub game: db::GameAndPlayers,
    pub winner_name: Option<String>,
}

// GameItemData should have list of player names
// It should also have stats:
// (number of finished games, number of wins, number of cancellations)
#[derive(Template)]
#[template(path="dashboard.html")]
pub struct DashboardTemplate {
    pub texts: DashTexts,
    pub user: auth::UserReqData,
    pub current_games: Vec<db::GameLinkData>,
    pub stats: db::PlayerStats,
    pub invited_game_hashes: Vec<String>,
}


#[derive(Template)]
#[template(path ="error.html")]
pub struct ErrorTemplate {
    pub error_data: ErrorData,
    pub user: auth::UserReqData,
    pub texts: ErrorTexts,
}



/* 
 * EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
 * EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
 * EEEEE                         EEEEE
 * EEEEE  OTHER STRUCTS & ENUMS  EEEEE
 * EEEEE                         EEEEE
 * EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
 * EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
*/



struct TwoAuthCookies {
    pub jwt_cookie: Cookie<'static>,
    pub refresh_token_cookie: Cookie<'static>,
}




/* 
 * 
 * 
 * 
 * 
 * ===========================
 * ===========================
 * =====                 =====
 * =====  ERROR RETURNS  =====
 * =====                 =====
 * ===========================
 * ===========================
 * 
 * 
 * 
 * 
*/



/**
 * Sometimes we don't know what went wrong and we need to return a JSON
 * object which says so.
 */
pub fn return_internal_err_json() -> HttpResponse {
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .json(ErrorResponse{
            error: String::from("Internal server error"),
            code: 500
        })
}

// If authentication failed and user must log back in
pub fn return_authentication_err_json() -> HttpResponse {
    HttpResponse::Unauthorized().json(ErrorResponse{
        error: String::from("Authentication required"),
        code: 401
    })
}


// If something is not found
pub fn return_not_found_err_json() -> HttpResponse {
    HttpResponse::Unauthorized().json(ErrorResponse{
        error: String::from("Not Found"),
        code: 406
    })
}

pub fn return_unauthorized_err_json(user_req_data: &auth::UserReqData) -> HttpResponse {
    let error: String = get_translation(
        "err.empty_creds",
        &user_req_data.lang,
        None
    );

    return HttpResponse::Unauthorized().json(
        ErrorResponse {
        error,
        code: 401
    });
}



/* 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  HELPER FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * 
*/

/**
 * Wrapping the DB's "finish game" function with a call to
 * "delete guesses" so the routes module can just
 * call this one function instead of both.
 */
pub async fn finish_game(
    pool: &MySqlPool,
    game_id: i32,
    winner_id_option: Option<i32>
) -> Result<u8> {
    let finish_game_affected_rows: u8 =
        db::finish_game(&pool, game_id, winner_id_option).await?;
    
    if finish_game_affected_rows > 0 {
        let _guesses_deleted: u8 = db::delete_guesses(pool, game_id).await?;
    }

    Ok(finish_game_affected_rows)
}

/**
 * Take a vector of GameId objects (each of which holds an i64 game id),
 * hash each one into a String, and return a vector of those Strings.
 */
pub fn get_hashes_from_game_ids(
    hash_ids: &web::Data<HashIds>,
    raw_ids: Vec<db::GameId>
) -> Vec<String> {
    raw_ids
        .iter()
        .map(|raw_invite|
            hash_ids.encode(&[raw_invite.game_id as u64]))
        .collect()
}