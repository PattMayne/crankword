extern crate rand;
// import commonly used items from the prelude:
use anyhow::{ Result, anyhow };
use sqlx::{ MySqlPool };
use time::{ OffsetDateTime };

use crate::{ game_logic, words_solutions };


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
    game_id: i32,
    status: game_logic::GameStatus,
    number_of_players: u8,
}


// Full data for one game
pub struct Game {
    pub id: i32,
    pub word: String,
    pub game_status: String,
    pub owner_id: i32,
    pub winner_id: Option<i32>,
    pub turn_user_id: Option<i32>,
    pub created_timestamp: OffsetDateTime,
}


pub struct GameAndPlayers {
    pub game: Game,
    pub player_ids: Vec<i32>,
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
    pub fn user_is_player(&self, user_id: i32) -> bool {
        self.player_ids.contains(&user_id)
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


pub async fn get_game_by_id(game_id: i32) -> Result<Game> {
    let pool: MySqlPool = create_pool().await?;

    let game: Game = sqlx::query_as!(
        Game,
        "SELECT id, word, game_status, owner_id, winner_id,
            turn_user_id, created_timestamp FROM games
            WHERE id = ?",
        game_id
    ).fetch_one(&pool).await?;

    Ok(game)
}


pub async fn get_players_by_game_id(game_id: i32) -> Result<Vec<i32>> {
    let pool: MySqlPool = create_pool().await?;
    let rows= sqlx::query!(
        "SELECT user_id FROM game_users WHERE game_id = ?",
        game_id
    ).fetch_all(&pool).await?;

    Ok(rows.into_iter().map(|row| row.user_id).collect())
}


pub async fn get_game_and_players(game_id: i32) -> Result<GameAndPlayers> {
    let game: Game = get_game_by_id(game_id).await?;
    let player_ids: Vec<i32> = get_players_by_game_id(game_id).await?;
    Ok(GameAndPlayers { game, player_ids })
}

pub async fn get_current_games(user_id: i32) -> Result<Vec<GameItemData>> {
    let pool: MySqlPool = create_pool().await?;
    let games: Vec<GameItemData> = Vec::new();

    // FIRST get each game_id in game_users for the user_id

    let game_ids: Vec<GameId> = sqlx::query_as!(
        GameId,
        "SELECT game_id FROM game_users
            WHERE user_id = ?",
        user_id
    ).fetch_all(&pool).await?;

    for game_id in game_ids {

        // Get each GAME object from the database

        // let game_result = sqlx::query_as!(
        //     RefreshToken,
        //     "SELECT id, game_status,
        //         turn_user_id, created_timestamp
        //         FROM games WHERE user_id = ? AND client_id = ?",
        //     user_id, client_id
        // ).fetch_optional(&pool).await?

    }




    // Ok(Some(GameItemData {
    //     game_id: 1,
    //     status: game_logic::GameStatus::PreGame,
    //     number_of_players: 2
    // }))

    Ok(games)
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

pub async fn new_game(owner_id: i32) -> Result<i32, anyhow::Error> {
    let pool: MySqlPool = create_pool().await?;

    // get word
    let word: String = words_solutions::get_random_word();

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO games (
            word,
            owner_id)
            VALUES (?, ?)")
        .bind(word)
        .bind(owner_id)
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
            user_id)
            VALUES (?, ?)")
        .bind(game_id)
        .bind(owner_id)
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