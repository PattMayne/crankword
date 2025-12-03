extern crate rand;
// import commonly used items from the prelude:
use rand::prelude::*;


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
 * Prototype / Interface
 * No DB yet. Just deliver the info.
 * 
*/



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