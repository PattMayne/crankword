extern crate rand;
// import commonly used items from the prelude:
use anyhow::{ Result, anyhow };
use sqlx::{MySqlPool };
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


// For referential items in a list of games
pub struct GameItemData {
    game_id: i32,
    status: game_logic::GameStatus,
    number_of_players: u8,
}


// Full data for one game
pub struct Game {
    id: i32,
    word: String,
    game_status: String,
    owner_id: i32,
    winner_id: Option<i32>,
    turn_user_id: Option<i32>,
    created_timestamp: OffsetDateTime,
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

pub async fn new_game(owner_id: i32) -> Result<i32, anyhow::Error> {
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

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

    Ok(result.last_insert_id() as i32)
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