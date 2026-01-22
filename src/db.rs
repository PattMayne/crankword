extern crate rand;
// import commonly used items from the prelude:
use anyhow::{ Result, anyhow };
use serde::Serialize;
use sqlx::{ MySqlPool };
use time::{ OffsetDateTime };
use rand::Rng;

use crate::{ auth, game_logic::{self, GameStatus, GuessAndScore, LetterScore, WordlessScore}, words_solutions };


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
struct GameId {
    game_id: i64,
}


struct UserId {
    user_id: i64,
}

// For referential items in a list of games
pub struct GameItemData {
    pub id: i32,
    pub game_status: String,
    pub winner_id: Option<i32>,
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



// raw DB data for one game to populate Game
pub struct RawGame {
    pub id: i32,
    pub word: String,
    pub game_status: String,
    pub owner_id: i32,
    pub winner_id: Option<i32>,
    pub turn_user_id: Option<i32>,
    pub created_timestamp: OffsetDateTime,
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
}

pub struct Guess {
    pub id: i64,
    pub word: String,
    pub game_id: i32,
    pub user_id: i32,
    pub guess_number: i8,
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
        }
    }
}


impl GameItemData {
    pub fn new_from(item: &GameItemData) -> GameItemData {
        GameItemData {
            id: item.id,
            game_status: item.game_status.to_owned(),
            winner_id: item.winner_id
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


// pub async fn get_player_stats(user_id: i32) -> Result<PlayerStats> {
//     let pool: MySqlPool = create_pool().await?;

// }


/**
 * All player's guesses for a given game.
 */
pub async fn get_guesses(game_id: i32, user_id: i32) -> Result<Vec<Guess>> {
    let pool: MySqlPool = create_pool().await?;
    let guesses: Vec<Guess> = sqlx::query_as!(
        Guess,
        "SELECT id, game_id, word, guess_number, user_id FROM guesses
            WHERE user_id = ? AND game_id = ?
            ORDER BY guess_number ASC",
        user_id, game_id
    ).fetch_all(&pool).await?;

    Ok(guesses)
}

/**
 * We're not directly calling the DB here.
 * Instead, we're calling other DB functions to collect some data and return it.
 * 
 * 1. get all words guessed by this user for this game
 * 2. get a vector of LetterScore structs for each guess
 * 3. return a vec of all those vecs
 */
pub async fn get_guess_scores(game_id: i32, user_id: i32) -> Result<Vec<game_logic::GuessAndScore>> {
    let the_game: Game = get_game_by_id(game_id).await?;
    let guesses: Vec<Guess> = get_guesses(game_id, user_id).await?;
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
    the_game: &Game,
    user_id: i32
) -> Result<Vec<game_logic::WordlessScore>> {
    // Get the full guess so we can get the score
    let guesses: Vec<Guess> = get_guesses(the_game.id, user_id).await?;
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
 * Returns number of PreGame or InProgress games the user is registered for.
 * (We're only allowed one game at a time)
 */
pub async fn get_current_games_count(user_id: i32) -> Result<u8> {
    let pool: MySqlPool = create_pool().await?;

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
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch games count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch games count: {e}"));
        }
    };

    let count: u8 = count_option.unwrap_or(Count{count: 0}).count as u8;

    Ok(count)
}



pub async fn get_guess_count(game_id: i32, user_id: i32) -> Result<u8> {
    let pool: MySqlPool = create_pool().await?;
    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM guesses WHERE game_id = ? AND user_id = ?",
        game_id,
        user_id
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch guesses count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch guesses count: {e}"));
        }
    };

    let count: u8 = count_option.unwrap_or(Count{count: 0}).count as u8;

    Ok(count)
}

pub async fn get_game_by_id(game_id: i32) -> Result<Game> {
    let pool: MySqlPool = create_pool().await?;

    // RawGame gets the string from game_status, all to populate Game which takes an enum.
    let raw_game: RawGame = sqlx::query_as!(
        RawGame,
        "SELECT id, word, game_status, owner_id, winner_id,
            turn_user_id, created_timestamp FROM games
            WHERE id = ?",
        game_id
    ).fetch_one(&pool).await?;

    Ok(Game::new(&raw_game))
}


pub async fn get_players_by_game_id(game_id: i32) -> Result<Vec<PlayerInfo>> {
    let pool: MySqlPool = create_pool().await?;

    let player_info_vec: Vec<PlayerInfo> = sqlx::query_as!(
        PlayerInfo,
        "SELECT user_id, username FROM game_users WHERE game_id = ?
            ORDER BY turn_order ASC",
        game_id
    ).fetch_all(&pool).await?;

    Ok(player_info_vec)
}


/**
 * This gets the WORDLESS guess scores along with basic player info
 * for displaying OPPONENT info on player's page during in-progress games.
 */
pub async fn get_players_refresh_data_by_game_id(game: &Game) -> Result<Vec<PlayerRefreshData>> {
    let pool: MySqlPool = create_pool().await?;

    // First just get the PlayerInfo
    let player_info_vec: Vec<PlayerInfo> = sqlx::query_as!(
        PlayerInfo,
        "SELECT user_id, username FROM game_users WHERE game_id = ?
            ORDER BY turn_order ASC",
        game.id
    ).fetch_all(&pool).await?;

    let mut players_refresh_data: Vec<PlayerRefreshData> = Vec::new();

    for player_info in player_info_vec {
        let scores: Vec<WordlessScore> = match get_wordless_guess_scores(&game, player_info.user_id).await {
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


pub async fn get_game_and_players(game_id: i32) -> Result<GameAndPlayers> {
    let game: Game = get_game_by_id(game_id).await?;
    let players: Vec<PlayerInfo> = get_players_by_game_id(game_id).await?;
    Ok(GameAndPlayers { game, players })
}

/**
 * Get basic data for all of user's current games.
 */
pub async fn get_current_games(user_id: i32) -> Result<Vec<GameItemData>> {
    let pool: MySqlPool = create_pool().await?;

    let games: Vec<GameItemData> = sqlx::query_as!(
        GameItemData,
        r#"
            SELECT g.id, g.game_status, g.winner_id
            FROM games g
            JOIN game_users gu ON g.id = gu.game_id
            WHERE gu.user_id = ?
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await?;


    // we have the games
    // now make the status object
    // and make the current games


    Ok(games)
}


/**
 * Check if any players still have turns left to play.
 */
pub async fn somebody_can_play(game_id: i32) -> Result<bool> {
    let players: Vec<PlayerInfo> = get_players_by_game_id(game_id).await?;
    for player in players {
        let guess_count: u8 = get_guess_count(game_id, player.user_id).await?;
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
    user_id: i32,
    game_id: i32,
    guess_word: &str,
    guess_number: u8
) -> Result<i64, anyhow::Error> {
    let pool: MySqlPool = create_pool().await?;
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO guesses (
            game_id, word, guess_number, user_id)
            VALUES (?, ?, ?, ?)")
        .bind(game_id)
        .bind(guess_word)
        .bind(guess_number)
        .bind(user_id)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save GUESS to database: {:?}", e);
            anyhow!("Could not save GUESS to database: {e}")
    })?;

    Ok(result.last_insert_id() as i64)
}

pub async fn new_game(user_req_data: &auth::UserReqData) -> Result<i32, anyhow::Error> {
    let pool: MySqlPool = create_pool().await?;

    // get word
    let word: String = words_solutions::get_random_word();

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO games (
            word,
            owner_id)
            VALUES (?, ?)")
        .bind(word)
        .bind(user_req_data.id)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save game to database: {:?}", e);
            anyhow!("Could not save game to database: {e}")
    })?;

    let game_id: i32 = result.last_insert_id() as i32;

    println!("game id {}", game_id);

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
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save game_user to database: {:?}", e);
            anyhow!("Could not save game_user to database: {e}")
    })?;

    println!("game_ id {}", game_id);

    if game_users_result.rows_affected() > 0 {
        Ok(game_id)
    } else {
        Err(anyhow!("Could not save game_user to database"))
    }
}


/**
 * User wants to join an existing game.
 * 
 * TODO:
 *  -- check that it's pre-game
 *  -- check that user isn't already in the game
 */
pub async fn user_join_game(
    user_req_data: &auth::UserReqData,
    game_id: i32
) -> Result<bool> {
    let pool: MySqlPool = create_pool().await?;
    if user_req_data.id.is_none() || user_req_data.get_role() == "guest" {
        return Err(anyhow!("User is not valid"));
    }

    // get the game and check that it's pregame
    // make sure the user isn't already part of the game

    let game: GameAndPlayers = get_game_and_players(game_id).await?;
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
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save game_user to database: {:?}", e);
            anyhow!("Could not save game_user to database: {e}")
    })?;

    Ok(result.rows_affected() > 0 )
}


/**
 * Get winning word from given game_id
 */
pub async fn get_winning_word(game_id: i32) -> Result<String> {
    let game: Game = get_game_by_id(game_id).await?;
    Ok(game.word)
}


pub async fn create_pool() -> Result<MySqlPool> {
     // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    let database_url: String = std::env::var("DATABASE_URL")?;
    Ok(MySqlPool::connect(database_url.as_str()).await?)
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
pub async fn next_turn(game_id: i32) -> Result<i32> {
    let pool: MySqlPool = create_pool().await?;
    let players: Vec<PlayerInfo> = get_players_by_game_id(game_id).await?;
    let game: Game = get_game_by_id(game_id).await?;
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

    let _result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE games SET turn_user_id = ? WHERE id = ?")
        .bind(new_user_turn_id)
        .bind(game_id)
        .execute(&pool)
        .await?;            

    Ok(new_user_turn_id)
}

/**
 * When transitioning a game from one stage to the next.
 */
pub async fn start_game(game_id: i32) -> Result<u8> {
    let pool: MySqlPool = create_pool().await?;
    let mut turn_user_id: i32 = 0;

    // set turn orders. Get all players. Scramble their IDs. Scrambled index +1 becomes turn order.
    let mut players: Vec<PlayerInfo> = get_players_by_game_id(game_id).await?;
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
            .execute(&pool)
            .await?;
    }

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE games SET game_status = ?, turn_user_id = ? WHERE id = ?")
        .bind(GameStatus::InProgress.to_string())
        .bind(turn_user_id)
        .bind(game_id)
        .execute(&pool)
        .await?;

    Ok(result.rows_affected() as u8)
}


/**
 * If we have a winner, send in Some(winner_id).
 * Else, everybody has lost.
 */
pub async fn finish_game(game_id: i32, winner_id_option: Option<i32>) -> Result<u8> {
    let pool: MySqlPool = create_pool().await?;

    match winner_id_option {
        Some(winner_id) => {
            let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "UPDATE games SET game_status = ?, winner_id = ? WHERE id = ?")
                .bind(GameStatus::Finished.to_string())
                .bind(winner_id)
                .bind(game_id)
                .execute(&pool)
                .await?;
            Ok(result.rows_affected() as u8)
        }, None => {
            let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "UPDATE games SET game_status = ? WHERE id = ?")
                .bind(GameStatus::Finished.to_string())
                .bind(game_id)
                .execute(&pool)
                .await?;
            Ok(result.rows_affected() as u8)
        }
    }
}