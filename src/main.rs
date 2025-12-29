#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{ App, HttpServer, middleware::{from_fn}, web };
use actix_files::Files;
use dotenvy;

mod routes;
mod game_logic;
mod db;
mod auth_code_shared;
mod auth;
mod io;
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
 * TO DO (in this order):
 * -- askama template for game
 * -- provide word from backend (hardcoded for now)
 * -- connect with backend to check guesses
 * -- create AUTH script for user to login
 * -- create single-player version for logged-in person
 * ---- create DB
 * ---- user can win game
 * -- create multi-player version
 * 
 * 
 * 
 * */


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenvy loads env variables for whole app
    // after this, just call std::env::var(variable_name)
    dotenvy::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .wrap(from_fn(middleware::login_status_middleware))
            .service(routes::error_root)
            .service(routes::error_root_2)
            .service(routes::error_page)
            .service(routes::home)
            .service(routes::game)
            .service(routes::login)
            .service(routes::register)
            .service(routes::logout)
            .service(routes::reception)
            .service(routes::dashboard)
            .service(
                web::scope("/game_in")
                .service(routes::check_word)
            )
            .default_service(web::get().to(routes::not_found)) // <- catch-all
            .wrap(from_fn(middleware::jwt_cookie_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


/*
 * ROUTES SCHEME:
 *      /game/{}            -- get user_id from JSON web token from cookie
 *      /game_in/           -- SCOPE for routes sending data TO the game (db) FROM the user/client ()POST
 *      /game_out/          -- SCOPE for routes sending data FROM the game (db) TO the user/client
 */


 fn check_words() {
    let word: &str = words_solutions::get_random_word();
    //let word: &str = "hghgh";
    println!("{}", word);
    let word_exists: bool = words_all::check_word(word);

    if word_exists {
        println!("{} exists", word);
    } else {
        println!("{} does NOT exist", word);
    }
 }