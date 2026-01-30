#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{ App, HttpServer, middleware::{from_fn}, web };
use actix_files::Files;
use dotenvy;
use std::io;
use hash_ids::HashIds;
use sqlx::{ MySqlPool };

mod routes;
mod routes_utils;
mod game_logic;
mod db;
mod auth_code_shared;
mod auth;
mod crankword_io;
mod utils;
mod middleware;
mod resources;
mod resource_mgr;
mod words_solutions;
mod words_all;

pub struct AppConfig {
    pub client_id: String,
}


/* 
 * ===========================
 * ===========================
 * =====                 =====
 * =====  MAIN FUNCTION  =====
 * =====                 =====
 * ===========================
 * ===========================
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Prepare data for storage in app data and other universal utils

    /* dotenvy loads env variables for whole app
     * after this, just call std::env::var(variable_name) */
    dotenvy::dotenv().ok();

    // Prepare the hash for hashing game_ids and user_ids
    let hash_ids: HashIds = match get_hashids().await {
        Ok(ids) => ids,
        Err(_e) => return hashid_secret_err().await
    };

    // Create the database pool that every function will use
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => pool,
        Err(_e) => return database_pool_err().await
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(hash_ids.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "./static"))
            .wrap(from_fn(middleware::login_status_middleware))
            .service(routes::error_root)
            .service(routes::error_root_2)
            .service(routes::error_page)
            .service(routes::home)
            .service(routes::game)
            .service(routes::game_root)
            .service(routes::new_game)
            .service(routes::login)
            .service(routes::register)
            .service(routes::logout)
            .service(routes::reception)
            .service(routes::dashboard)
            .service(
                web::scope("/game_in")
                .service(routes::check_guess)
                .service(routes::join_game)
                .service(routes::start_game)
                .service(routes::refresh_pregame)
                .service(routes::refresh_in_prog_players)
                .service(routes::get_guess_scores)
                .service(routes::invite_player)
            )
            .default_service(web::get().to(routes::not_found)) // <- catch-all
            .wrap(from_fn(middleware::jwt_cookie_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn hashid_secret_err() -> std::io::Result<()> {
    eprintln!("ERROR: NO HASH ID SECRET.");
    return Err(
        io::Error::new(
            io::ErrorKind::NotFound, "HASHID_SECRET not set")
    );
}


async fn database_pool_err() -> std::io::Result<()> {
    eprintln!("ERROR: NO HASH ID SECRET.");
    return Err(
        io::Error::new(
            io::ErrorKind::Other, "HASHID_SECRET not set")
    );
}

/*
 * ROUTES SCHEME:
 *      /game/{}            -- get user_id from JSON web token from cookie
 *      /game_in/           -- SCOPE for routes sending data TO the game (db) FROM the user/client ()POST
 *      /game_out/          -- SCOPE for routes sending data FROM the game (db) TO the user/client
 */


 fn check_words() {
    let word: String = words_solutions::get_random_word();
    println!("{}", word);
    let word_exists: bool = words_all::is_real_word(&word);

    if word_exists {
        println!("{} exists", word);
    } else {
        println!("{} does NOT exist", word);
    }
 }


 /**
  * Create the database thread pool that every function will use
  */
async fn create_pool() -> Result<MySqlPool, String> {
    let database_url: String = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_e) => return Err("Database Error".to_string())
    };

    let pool = match MySqlPool::connect(database_url.as_str()).await {
        Ok(pool) => pool,
        Err(_e) => return Err("Database Error".to_string())
    };
    
    Ok(pool)
}


// Prepare the hash for hashing game_ids and user_ids
async fn get_hashids() -> Result<HashIds, String> {
    let hashid_secret: String = match std::env::var("HASHID_SECRET") {
        Ok(secret) => secret,
        Err(_e) => return Err("HASHID_SECRET not set".to_string())
    };

    let hash_ids: HashIds = HashIds::builder()
        .with_salt(&hashid_secret)
        .finish();

    Ok(hash_ids)
}