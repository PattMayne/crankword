use serde::{ Serialize };
use std::collections::BTreeMap;

    
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LetterScore {
    RightSpot,
    WrongSpot,
    Dud,
}
    
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    PreGame,
    InProgress,
    Finished,
    Cancelled,
}

/**
 * We need occurrences of each letter so we can highlight the correct
 * number of occurrences in the guess.
 */
fn get_letter_counts(word: &String) -> BTreeMap<char, u8> {
    let mut letter_counts: BTreeMap<char, u8> = BTreeMap::new();

    for letter in word.chars() {
        letter_counts.insert(letter, {
            match letter_counts.get(&letter) {
                None => 1,
                Some(count) => count +1
            }
        });
    }

    letter_counts
}


/**
 * Checking for perfect placement.
 * Simple true/false return.
 * No side effects here.
 */
fn letter_is_right_spot(
    letter: &char,
    winning_word: &String,
    input_position: u8,
    letter_counts: &mut BTreeMap<char, u8>
) -> bool {
    
    let letter_at_position_option: Option<char> =
        winning_word.chars().nth(input_position as usize);

    if letter_at_position_option.is_none() {
        eprintln!("LETTER POSITION OUT OF RANGE");
        return false;
    }

    if letter_at_position_option.unwrap() == *letter {
        let count_option: Option<&u8> = letter_counts.get(letter);
        if count_option.is_none() {
            return false;
        }

        if *count_option.unwrap() > 0 {
            return true;
        }
    }

    false
}


/**
 * Send in the guess word and the winning word,
 * we do the comparison and build a map of results
 * about the status of each letter in the guess word.
 */
pub fn check_guess(
    guess_word: &String,
    winning_word: &String
) -> Vec<LetterScore> {
    // Create default vector with size based on guess word size
    let mut guess_map: Vec<LetterScore> = Vec::new();
    for _ in guess_word.chars() {
        guess_map.push(LetterScore::Dud);
    }

    let mut letter_counts: BTreeMap<char, u8> = get_letter_counts(winning_word);

    // Check each letter for perfect placement
    for (input_position, letter) in guess_word.chars().enumerate() {
        let letter_is_right_spot: bool = letter_is_right_spot(
            &letter,
            &winning_word,
            input_position as u8,
            &mut letter_counts
        );

        if letter_is_right_spot && input_position < guess_map.len() {
            let count_option: Option<&u8>  = letter_counts.get(&letter);
            if count_option.is_none() {
                continue
            }

            let count: u8 = *count_option.unwrap();
            guess_map[input_position] = LetterScore::RightSpot;

            // decrement count
            letter_counts.insert(letter.clone(), count - 1);
        }
    }

    // Check each letter for wrong position or else dud
    for (input_position, letter) in guess_word.chars().enumerate() {
        // Skip the ones that have already been found in the right spot
        if guess_map[input_position] == LetterScore::RightSpot {
            continue
        }

        match letter_counts.get(&letter) {
            None => continue, // DUD
            Some(count) => {
                if *count > 0 {
                    // Good letter in the wrong spot
                    letter_counts.insert(letter, count - 1);
                    guess_map[input_position] = LetterScore::WrongSpot;
                }
            }
        }
    }
    
    guess_map
}