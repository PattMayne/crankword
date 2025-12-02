#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{ App, HttpServer, middleware::{from_fn}, web };
use actix_files::Files;
use dotenvy;


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


fn main() {
    println!("Hello, world!");
}
