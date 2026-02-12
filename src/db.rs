extern crate rand;
// import commonly used items from the prelude:
use anyhow::{ Result, anyhow };
use serde::Serialize;
use sqlx::{ MySqlPool, MySql, Transaction };
use rand::Rng;
use time::{ OffsetDateTime };

use crate::{
    auth,
    words_solutions,
    game_logic::{
        self,
        GameStatus,
        GuessAndScore,
        LetterScore,
        WordlessScore
    }
};


/* 
 * 
 * 
 * 
 * 
 * ======================
 * ======================
 * =====            =====
 * =====  DATABASE  =====
 * =====            =====
 * ======================
 * ======================
 * 
 * 
 * 
*/


/* 
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

// For retrieving game_id from the game_users table
pub struct GameId {
    pub game_id: i64,
}

pub struct GameIdAndOwnerName {
    pub game_id: i64,
    pub owner_name: String,
}


struct UserId {
    user_id: i64,
}

// For referential items in a list of games
pub struct GameItemData {
    pub id: i32,
    pub game_status: String,
    pub winner_id: Option<i32>,
    pub created_timestamp: OffsetDateTime,
}

pub struct GameLinkData {
    pub hashid: String,
    pub game_status: String,
    pub age_string: String,
}


#[derive(Serialize)]
pub struct RawOpenGame {
    pub id: i32,
    pub created_timestamp: OffsetDateTime,
}


#[derive(Debug)]
struct Count {
    count: i64,
}



#[derive(Serialize)]
pub struct PlayerStats {
    pub wins: u32,
    pub past_games: u32,
    pub cancelled_games: u32,
}

pub struct InviteeUsername {
    pub username: String,
}


// raw DB data for one game to populate Game
pub struct RawGame {
    pub id: i32,
    pub word: String,
    pub game_status: String,
    pub owner_id: i32,
    pub winner_id: Option<i32>,
    pub turn_user_id: Option<i32>,
    pub created_timestamp: OffsetDateTime,
    pub turn_timeout: OffsetDateTime,
    pub open_game: i8,
}

// Full data for one game
pub struct Game {
    pub id: i32,
    pub word: String,
    pub game_status: GameStatus,
    pub owner_id: i32,
    pub winner_id: Option<i32>,
    pub turn_user_id: Option<i32>,
    pub created_timestamp: OffsetDateTime,
    pub turn_timeout: OffsetDateTime,
    pub open_game: bool,
}

pub struct Guess {
    pub id: i64,
    pub word: String,
    pub game_id: i32,
    pub user_id: i32,
    pub guess_number: i8,
    pub created_timestamp: OffsetDateTime,
}


/* Simple identifiers for a Player  */
#[derive(PartialEq, Serialize)]
pub struct PlayerInfo {
    pub user_id: i32,
    pub username: String,
}

/* Simple identifiers for a Player  */
#[derive(PartialEq, Serialize)]
pub struct PlayerRefreshData {
    pub user_id: i32,
    pub username: String,
    pub scores: Vec<WordlessScore>,
}


pub struct GameAndPlayers {
    pub game: Game,
    pub players: Vec<PlayerInfo>,
}


impl GameAndPlayers {

    pub fn owner_name(&self) -> Option<&String> {
        for player in &self.players {
            if self.game.owner_id == player.user_id {
                return Some(&player.username)
            }
        }

        None
    }
}


impl Game {
    pub fn new(raw_game: &RawGame) -> Self {
        Game {
            id: raw_game.id,
            word: raw_game.word.to_owned(),
            game_status: GameStatus::from_string(&raw_game.game_status),
            owner_id: raw_game.owner_id,
            winner_id: raw_game.winner_id,
            turn_user_id: raw_game.turn_user_id,
            created_timestamp: raw_game.created_timestamp,
            turn_timeout: raw_game.turn_timeout,
            open_game: raw_game.open_game == 1
        }
    }
}


impl GameItemData {
    pub fn new_from(item: &GameItemData) -> GameItemData {
        GameItemData {
            id: item.id,
            game_status: item.game_status.to_owned(),
            winner_id: item.winner_id,
            created_timestamp: item.created_timestamp
        }
    }
}


impl UserId {
    pub fn new(id: i32) -> UserId {
        UserId { user_id: id as i64 }
    }

    pub fn get_id(&self) -> i64 {
        self.user_id
    }
}

impl GameId {
    pub fn new(id: i32) -> GameId {
        GameId { game_id: id as i64 }
    }

    pub fn get_id(&self) -> i64 {
        self.game_id
    }
}

impl GameAndPlayers {
    pub fn user_is_player(&self, player_info: PlayerInfo) -> bool {
        self.players.contains(&player_info)
    }

    pub fn user_id_is_player(&self, player_id: i32) -> bool {
        for player_info in &self.players {
            if player_info.user_id == player_id {
                return true;
            }
        }

        false
    }

    pub fn username_is_player(&self, username: &String) -> bool {
        for player_info in &self.players {
            if player_info.username == username.to_string() {
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
 * ====================
 * ====================
 * =====          =====
 * =====  SELECT  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * 
*/


pub async fn is_blocked(
    pool: &MySqlPool,
    blocker_username: &String,
    blocked_username: &String
) -> Result<bool> {
    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM blocks WHERE blocker_username = ? AND blocked_username = ?",
        blocker_username,
        blocked_username
    ).fetch_optional(pool).await {
        Ok(count) => count,
        Err(e) => return Err(anyhow!("Could not fetch invites count: {e}"))
    };

    match count_option {
        Some(count) => Ok(count.count > 0),
        None => Err(anyhow!("Could not fetch invites count"))
    }
}



/**
 * Get winning word from given game_id
 */
pub async fn get_winning_word(pool: &MySqlPool, game_id: i32) -> Result<String> {
    let game: Game = get_game_by_id(pool, game_id).await?;
    Ok(game.word)
}


/**
 * When we need a list of invitee names for the pre-game dashboard.
 */
pub async fn get_invitee_usernames(pool: &MySqlPool, game_id: i32) -> Result<Vec<String>> {
    let invitee_usernames: Vec<InviteeUsername> = sqlx::query_as!(
        InviteeUsername,
        "SELECT username FROM invites WHERE game_id = ?",
        game_id
    ).fetch_all(pool).await?;

    let mut usernames: Vec<String> = Vec::new();
    for invitee_username in invitee_usernames {
        usernames.push(invitee_username.username.to_string())
    }

    Ok(usernames)
}


pub async fn get_invitations_by_username(
    pool: &MySqlPool,
    username: String
) -> Result<Vec<GameIdAndOwnerName>> {
    let game_ids: Vec<GameIdAndOwnerName> = sqlx::query_as!(
        GameIdAndOwnerName,
        "SELECT game_id, owner_name FROM invites WHERE username = ?",
        username
    ).fetch_all(pool).await?;

    Ok(game_ids)
}


/**
 * All player's guesses for a given game.
 */
pub async fn get_guesses(pool: &MySqlPool, game_id: i32, user_id: i32) -> Result<Vec<Guess>> {
    let guesses: Vec<Guess> = sqlx::query_as!(
        Guess,
        "SELECT id, game_id, word, guess_number, user_id, created_timestamp FROM guesses
            WHERE user_id = ? AND game_id = ?
            ORDER BY guess_number ASC",
        user_id, game_id
    ).fetch_all(pool).await?;

    Ok(guesses)
}


/**
 * Get all games which are marked "open".
 */
pub async fn get_open_games(pool: &MySqlPool) -> Result<Vec<RawOpenGame>> {
    let games: Vec<RawOpenGame> = sqlx::query_as!(
        RawOpenGame,
        "SELECT id, created_timestamp FROM games
            WHERE open_game = ? AND game_status = ?
            ORDER BY created_timestamp ASC",
            1, GameStatus::PreGame.to_string()
    ).fetch_all(pool).await?;

    Ok(games)
}


/**
 * We're not directly calling the DB here.
 * Instead, we're calling other DB functions to collect some data and return it.
 * 
 * 1. get all words guessed by this user for this game
 * 2. get a vector of LetterScore structs for each guess
 * 3. return a vec of all those vecs
 */
pub async fn get_guess_scores(
    pool: &MySqlPool,
    game_id: i32,
    user_id: i32
) -> Result<Vec<game_logic::GuessAndScore>> {
    let the_game: Game = get_game_by_id(pool, game_id).await?;
    let guesses: Vec<Guess> = get_guesses(pool, game_id, user_id).await?;
    let all_scores: Vec<game_logic::GuessAndScore> = 
        guesses
        .iter()
        .map(
            |guess| {
                let word: String = guess.word.to_string();
                let score: Vec<LetterScore> =
                    game_logic::check_guess(
                        &guess.word, &the_game.word
                    ).score;

                GuessAndScore {word, score}
            }
        ).collect();

    Ok(all_scores)
}

/**
 * We're not directly calling the DB here.
 * Instead, we're calling other DB functions to collect some data and return it.
 * We want to ONLY deliver the score, because this is FOR DISPLAY for the other players.
 * 
 * 1. get all words guessed by this user for this game
 * 2. get a vector of LetterScore structs for each guess
 * 3. return a vec of all those vecs
 */
pub async fn get_wordless_guess_scores(
    pool: &MySqlPool,
    the_game: &Game,
    user_id: i32
) -> Result<Vec<game_logic::WordlessScore>> {
    // Get the full guess so we can get the score
    let guesses: Vec<Guess> = get_guesses(pool, the_game.id, user_id).await?;
    // Deliver only the score, without the word
    let all_scores: Vec<game_logic::WordlessScore> = 
        guesses
        .iter()
        .map(
            |guess| {
                let score: Vec<LetterScore> =
                    game_logic::check_guess(
                        &guess.word, &the_game.word
                    ).score;

                game_logic::WordlessScore { score }
            }
                
        ).collect();

    Ok(all_scores)
}


/**
 * Check how many invitations have been sent for this game.
 */
pub async fn get_invites_count(pool: &MySqlPool, game_id: i32) -> Result<u8> {
    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM invites WHERE game_id = ?",
        game_id
    ).fetch_optional(pool).await {
        Ok(count) => count,
        Err(e) => return Err(anyhow!("Could not fetch invites count: {e}"))
    };

    match count_option {
        Some(count) => Ok(count.count as u8),
        None => Ok(0)
    }
}


/**
 * Check how many invitations have been sent for this game.
 */
pub async fn get_game_players_count(pool: &MySqlPool, game_id: i32) -> Result<u8> {
    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM game_users WHERE game_id = ?",
        game_id
    ).fetch_optional(pool).await {
        Ok(count) => count,
        Err(e) => return Err(anyhow!("Could not fetch players count: {e}"))
    };

    match count_option {
        Some(count) => Ok(count.count as u8),
        None => Ok(0)
    }
}



/**
 * Returns number of PreGame or InProgress games the user is registered for.
 * (We're only allowed one game at a time)
 */
pub async fn get_current_games_count(pool: &MySqlPool, user_id: i32) -> Result<u8> {

    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM games g
        JOIN game_users gu ON g.id = gu.game_id
        WHERE gu.user_id = ?
        AND (g.game_status = ?
        OR g.game_status = ?)",
        user_id,
        GameStatus::PreGame.to_string(),
        GameStatus::InProgress.to_string()
    ).fetch_optional(pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch games count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch games count: {e}"));
        }
    };

    let count: u8 = count_option.unwrap_or(Count{count: 0}).count as u8;

    Ok(count)
}



pub async fn get_guess_count(pool: &MySqlPool, game_id: i32, user_id: i32) -> Result<u8> {
    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM guesses WHERE game_id = ? AND user_id = ?",
        game_id,
        user_id
    ).fetch_optional(pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch guesses count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch guesses count: {e}"));
        }
    };

    let count: u8 = count_option.unwrap_or(Count{count: 0}).count as u8;

    Ok(count)
}

pub async fn get_game_by_id(pool: &MySqlPool, game_id: i32) -> Result<Game> {
    // RawGame gets the string from game_status, all to populate Game which takes an enum.
    let raw_game: RawGame = sqlx::query_as!(
        RawGame,
        "SELECT id, word, game_status, owner_id, winner_id, open_game,
            turn_user_id, turn_timeout, created_timestamp FROM games
            WHERE id = ?",
        game_id
    ).fetch_one(pool).await?;

    Ok(Game::new(&raw_game))
}


pub async fn get_players_by_game_id(pool: &MySqlPool, game_id: i32) -> Result<Vec<PlayerInfo>> {
    let player_info_vec: Vec<PlayerInfo> = sqlx::query_as!(
        PlayerInfo,
        "SELECT user_id, username FROM game_users WHERE game_id = ?
            ORDER BY turn_order ASC",
        game_id
    ).fetch_all(pool).await?;

    Ok(player_info_vec)
}


/**
 * This gets the WORDLESS guess scores along with basic player info
 * for displaying OPPONENT info on player's page during in-progress games.
 */
pub async fn get_players_refresh_data_by_game_id(
    pool: &MySqlPool,
    game: &Game
) -> Result<Vec<PlayerRefreshData>> {
    // First just get the PlayerInfo
    let player_info_vec: Vec<PlayerInfo> = sqlx::query_as!(
        PlayerInfo,
        "SELECT user_id, username FROM game_users WHERE game_id = ?
            ORDER BY turn_order ASC",
        game.id
    ).fetch_all(pool).await?;

    let mut players_refresh_data: Vec<PlayerRefreshData> = Vec::new();

    for player_info in player_info_vec {
        let scores: Vec<WordlessScore> = match get_wordless_guess_scores(pool, &game, player_info.user_id).await {
            Ok(scores) => scores,
            Err(_) => Vec::new()
        };

        players_refresh_data.push(PlayerRefreshData {
            user_id: player_info.user_id,
            username: player_info.username,
            scores,
        });
    }

    Ok(players_refresh_data)
}


pub async fn get_game_and_players(pool: &MySqlPool, game_id: i32) -> Result<GameAndPlayers> {
    let game: Game = get_game_by_id(pool, game_id).await?;
    let players: Vec<PlayerInfo> = get_players_by_game_id(pool, game_id).await?;
    Ok(GameAndPlayers { game, players })
}

/**
 * Get basic data for all of user's games.
 */
pub async fn get_games_byid(
    pool: &MySqlPool,
    user_id: i32
) -> Result<Vec<GameItemData>> {
    let games: Vec<GameItemData> = sqlx::query_as!(
        GameItemData,
        r#"
            SELECT g.id, g.game_status, g.winner_id, g.created_timestamp
            FROM games g
            JOIN game_users gu ON g.id = gu.game_id
            WHERE gu.user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(games)
}

/**
 * Get basic data for all of user's current games.
 */
pub async fn get_games_by_username(
    pool: &MySqlPool,
    username: &String
) -> Result<Vec<GameItemData>> {
    let games: Vec<GameItemData> = sqlx::query_as!(
        GameItemData,
        r#"
            SELECT g.id, g.game_status, g.winner_id, g.created_timestamp
            FROM games g
            JOIN game_users gu ON g.id = gu.game_id
            WHERE gu.username = ?
        "#,
        username
    )
    .fetch_all(pool)
    .await?;

    Ok(games)
}



/**
 * Check if any players still have turns left to play.
 */
pub async fn somebody_can_play(pool: &MySqlPool, game_id: i32) -> Result<bool> {
    let players: Vec<PlayerInfo> = get_players_by_game_id(pool, game_id).await?;
    for player in players {
        let guess_count: u8 = get_guess_count(pool, game_id, player.user_id).await?;
        if guess_count < game_logic::MAX_TURNS {
            return Ok(true)
        }
    }

    Ok(false)
}

/* 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  INSERT FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * 
*/

pub async fn new_guess(
    pool: &MySqlPool,
    user_id: i32,
    game_id: i32,
    guess_word: &str,
    guess_number: u8
) -> Result<i64, anyhow::Error> {
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO guesses (
            game_id, word, guess_number, user_id)
            VALUES (?, ?, ?, ?)")
        .bind(game_id)
        .bind(guess_word)
        .bind(guess_number)
        .bind(user_id)
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save GUESS to database: {:?}", e);
            anyhow!("Could not save GUESS to database: {e}")
    })?;

    Ok(result.last_insert_id() as i64)
}

pub async fn new_game(
    pool: &MySqlPool,
    user_req_data: &auth::UserReqData,
    open_game_bool: bool
) -> Result<i32, anyhow::Error> {
    // get word
    let word: String = words_solutions::get_random_word();
    let open_game_int: i32 = if open_game_bool { 1 } else { 0 };

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO games (
            word, owner_id, open_game)
            VALUES (?, ?, ?)")
        .bind(word)
        .bind(user_req_data.id)
        .bind(open_game_int)
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save game to database: {:?}", e);
            anyhow!("Could not save game to database: {e}")
    })?;

    let game_id: i32 = result.last_insert_id() as i32;

    // Now put owner_id in game_users table
    let game_users_result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO game_users (
            game_id,
            user_id,
            username)
            VALUES (?, ?, ?)")
        .bind(game_id)
        .bind(user_req_data.id)
        .bind(user_req_data.get_username())
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save game_user to database: {:?}", e);
            anyhow!("Could not save game_user to database: {e}")
    })?;

    if game_users_result.rows_affected() > 0 {
        Ok(game_id)
    } else {
        Err(anyhow!("Could not save game_user to database"))
    }
}


/**
 * User wants to join an existing game.
 */
pub async fn user_join_game(
    pool: &MySqlPool,
    user_req_data: &auth::UserReqData,
    game_id: i32
) -> Result<bool> {
    if user_req_data.id.is_none() || user_req_data.get_role() == "guest" {
        return Err(anyhow!("User is not valid"));
    }

    // get the game and check that it's pregame
    // make sure the user isn't already part of the game

    let game: GameAndPlayers = get_game_and_players(pool, game_id).await?;
    if game.game.game_status != GameStatus::PreGame {
        return Err(anyhow!("Game already started"));
    } else if game.user_id_is_player(user_req_data.id.unwrap()) {
        return Err(anyhow!("User already joined game."));
    }

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO game_users (
            game_id,
            user_id,
            username)
            VALUES (?, ?, ?)")
        .bind(game_id)
        .bind(user_req_data.id.unwrap())
        .bind(user_req_data.get_username())
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save game_user to database: {:?}", e);
            anyhow!("Could not save game_user to database: {e}")
    })?;

    Ok(result.rows_affected() > 0 )
}

/**
 * Invite user to an existing game.
 */
pub async fn invite_user(
    pool: &MySqlPool,
    username: &String,
    owner_name: &String,
    game_id: i32
) -> Result<bool> {
    // get the game and check that it's pregame
    // make sure the user isn't already part of the game

    let game: GameAndPlayers = get_game_and_players(pool, game_id).await?;
    if game.game.game_status != GameStatus::PreGame {
        return Err(anyhow!("Game already started"));
    } else if game.username_is_player(username) {
        return Err(anyhow!("User already joined game."));
    }

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO invites (
            game_id,
            username,
            owner_name)
            VALUES (?, ?, ?)")
        .bind(game_id)
        .bind(username)
        .bind(owner_name)
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save game_user to database: {:?}", e);
            anyhow!("Could not save game_user to database: {e}")
    })?;

    Ok(result.rows_affected() > 0 )
}


pub async fn block_user(
    pool: &MySqlPool,
    blocker_username: &String,
    blocked_username: &String
) -> Result<bool, anyhow::Error> {
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO blocks (
            blocker_username, blocked_username)
            VALUES (?, ?)")
        .bind(blocker_username)
        .bind(blocked_username)
        .execute(pool).await.map_err(|e| {
            eprintln!("Failed to save GUESS to database: {:?}", e);
            anyhow!("Could not save GUESS to database: {e}")
    })?;

    Ok(result.rows_affected() > 0)
}


/* 
 * 
 * 
 * 
 * 
 * ====================
 * ====================
 * =====          =====
 * =====  UPDATE  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * 
*/




/**
 * Returns the id of the new current turn user.
 */
pub async fn next_turn(pool: &MySqlPool, game_id: i32) -> Result<i32> {
    let players: Vec<PlayerInfo> = get_players_by_game_id(pool, game_id).await?;
    let game: Game = get_game_by_id(pool, game_id).await?;
    let current_user_id: i32 = match game.turn_user_id {
        Some(id) => id,
        None => return Err(anyhow!("No current turn."))
    };

    // get the vector index of the current turn
    let mut index_count: usize = 0;
    for player in &players {
        if player.user_id == current_user_id {
            break;
        } else {
            index_count += 1;
        }
    }

    // increment that index and get THAT user_id
    let vec_index_of_new_turn_id: usize =
        if index_count >= (players.len() - 1) { 0 }
        else { index_count + 1 };

    let new_user_turn_id: i32 = players[vec_index_of_new_turn_id].user_id;
    let turn_timeout: OffsetDateTime = game_logic::get_turn_timeout();

    let _result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE games SET turn_user_id = ?, turn_timeout = ? WHERE id = ?")
        .bind(new_user_turn_id)
        .bind(turn_timeout)
        .bind(game_id)
        .execute(pool)
        .await?;            

    Ok(new_user_turn_id)
}

/**
 * When transitioning a game from one stage to the next.
 */
pub async fn start_game(pool: &MySqlPool, game_id: i32) -> Result<bool> {
    let mut turn_user_id: i32 = 0;

    // set turn orders. Get all players. Scramble their IDs. Scrambled index +1 becomes turn order.
    let mut players: Vec<PlayerInfo> = get_players_by_game_id(pool, game_id).await?;
    let mut scrambled_player_ids: Vec<i32> = Vec::new();
    let number_of_players: usize = players.len();

    while scrambled_player_ids.len() < number_of_players {
        let index: usize = rand::rng().random_range(0..players.len());
        scrambled_player_ids.push(players[index].user_id);
        turn_user_id = players[index].user_id;
        players.remove(index);
    }

    // Instead of using an index, just increment turn during the loop.
    let mut turn: i32 = 0;

    // insert the indexes as turn orders into game_users table
    for user_id in &scrambled_player_ids {
        turn += 1;
        let _result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "UPDATE game_users SET turn_order = ? WHERE game_id = ? and user_id = ?")
            .bind(turn)
            .bind(game_id)
            .bind(user_id)
            .execute(pool)
            .await?;
    }

    let turn_timeout: OffsetDateTime = game_logic::get_turn_timeout();

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE games SET game_status = ?, turn_user_id = ?, turn_timeout = ? WHERE id = ?")
        .bind(GameStatus::InProgress.to_string())
        .bind(turn_user_id)
        .bind(turn_timeout)
        .bind(game_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}


/**
 * If we have a winner, send in Some(winner_id).
 * Else, everybody has lost.
 */
pub async fn finish_game(
    pool: &MySqlPool,
    game_id: i32,
    winner_id_option: Option<i32>
) -> Result<u8> {
    match winner_id_option {
        Some(winner_id) => {
            let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "UPDATE games SET game_status = ?, winner_id = ? WHERE id = ?")
                .bind(GameStatus::Finished.to_string())
                .bind(winner_id)
                .bind(game_id)
                .execute(pool)
                .await?;
            Ok(result.rows_affected() as u8)
        }, None => {
            let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "UPDATE games SET game_status = ? WHERE id = ?")
                .bind(GameStatus::Finished.to_string())
                .bind(game_id)
                .execute(pool)
                .await?;
            Ok(result.rows_affected() as u8)
        }
    }
}

/**
 * If we have a winner, send in Some(winner_id).
 * Else, everybody has lost.
 */
pub async fn cancel_game(
    pool: &MySqlPool,
    game_id: i32
) -> Result<bool> {
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE games SET game_status = ? WHERE id = ?")
        .bind(GameStatus::Cancelled.to_string())
        .bind(game_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/**
 * A user wants to quit, presumably because the game is dead and they need
 * to free up the space. So we will delete one game_users.
 * */
pub async fn remove_player_from_game(
    pool: &MySqlPool,
    game_id: i32,
    user_id: i32
) -> Result<bool> {

    let mut tx: Transaction<MySql> = pool.begin().await?;

    // Delete guesses for the player in the game
    sqlx::query("DELETE FROM guesses WHERE game_id = ? AND user_id = ?")
        .bind(game_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    // Delete the user from the game_users table
    let result = sqlx::query("DELETE FROM game_users WHERE game_id = ? AND user_id = ?")
        .bind(game_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();

    // Commit the transaction
    tx.commit().await?;

    Ok(result > 0)
}


/* 
 * 
 * 
 * 
 * 
 * ====================
 * ====================
 * =====          =====
 * =====  DELETE  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * 
*/

pub async fn delete_invite(
    pool: &MySqlPool,
    game_id: i32,
    username: &String
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM invites WHERE game_id = ? AND username = ?")
        .bind(game_id)
        .bind(username)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
}

pub async fn delete_invites(
    pool: &MySqlPool,
    game_id: i32
) -> Result<u8> {
    let result = sqlx::query(
        "DELETE FROM invites WHERE game_id = ?")
        .bind(game_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() as u8)
}


pub async fn delete_guesses(
    pool: &MySqlPool,
    game_id: i32
) -> Result<u8> {
    let result = sqlx::query(
        "DELETE FROM guesses WHERE game_id = ?")
        .bind(game_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() as u8)
}


pub async fn delete_user_from_game(
    pool: &MySqlPool,
    game_id: i32,
    username: &String
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM game_users WHERE game_id = ? AND username = ?")
        .bind(game_id)
        .bind(username)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
}


pub async fn delete_block(
    pool: &MySqlPool,
    blocker_username: &String,
    blocked_username: &String
) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM blocks WHERE blocker_username = ? AND blocked_username = ?")
        .bind(blocker_username)
        .bind(blocked_username)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
}