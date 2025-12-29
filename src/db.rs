extern crate rand;
// import commonly used items from the prelude:
use rand::prelude::*;
use anyhow::{ Result, anyhow, Context };
use sqlx::{MySqlPool };
use time::{ OffsetDateTime, Duration };

use crate::game_logic;

// For retrieving game_id from the game_users table
struct GameId {
    game_id: i64,
}

pub struct GameItemData {
    game_id: i32,
    status: game_logic::GameStatus,
    number_of_players: u8,
}

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


pub async fn get_current_games(user_id: i32) -> Result<Vec<GameItemData>> {

    let games: Vec<GameItemData> = Vec::new();

    let pool: MySqlPool = create_pool().await?;

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



pub async fn get_winning_word(game_id: i32) -> String {
    // list of words


    let words: Vec<&str> = Vec::from([
        "CRANK",
        "APPLE",
        "BAKER",
        "SMASH",
        "DONUT",
        "FOLLY",
        "TRASH",
        "MANGO",
        "BERRY",
        "MOVIE",
        "CAMEL",
        "CROSS",
        "GROSS",
        "DROSS",
        "COAST",
        "TOTAL",
        "FINAL",
        "HAPPY",
        "IMPLY",
        "TONER",
        "SOUPY",
        "GROPE",
        "STYLE",
        "VINYL",
        "CORAL",
        "STOUT",
        "SWORD",
        "BEVEL",
        "YOUTH"
    ]);

    // get one randomly (for new game)
    let mut rng: ThreadRng = rand::rng();
    let rand_word_index: usize = rng.random_range(0..words.len());
    let rand_word: &str = words[rand_word_index];
    println!("random word: {}", rand_word);

    // Retrieve from "storage"
    let stored_word_index: i32 =
        if game_id < words.len() as i32 &&
            game_id >= 0 { game_id }
        else { 5 };
    
    // return it (ignoring randomly chosen until we have a DB)
    return words[stored_word_index as usize].to_string();
}


pub async fn create_pool() -> Result<MySqlPool> {
     // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    let database_url: String = std::env::var("DATABASE_URL")?;
    Ok(MySqlPool::connect(database_url.as_str()).await?)
}