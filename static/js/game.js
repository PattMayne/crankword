$(document).foundation()
import * as io from './io.js'
import { LetterState, Tile } from './utils.js'

/* 
 * 
 * 
 * 
 * 
 * =======================
 * =======================
 * =====             =====
 * =====  CRANKWORD  =====
 * =====             =====
 * =======================
 * =======================
 * 
 * 
 *
 * Front-end prototype for a word-guessing game.
 * Will be multi-player.
 * 
 * 
 *
 * 
*/


/* 
 * ==========================
 * =====                =====
 * =====  window stuff  =====
 * =====                =====
 * ==========================
*/


const board_panel = document.getElementById("board-panel")
const board = document.getElementById("board")
const headline = document.getElementById("headline")
const message_modal = $('#message_modal') // Foundation demands jquery for this
const message_p = document.getElementById("message_p")

window.addEventListener("load", () => start_game())
window.addEventListener("keydown", (event) => key_pressed(event));

// Debounce the resizer. (Avoids constantly resizing as user adjusts screen size)
let timeout;
window.addEventListener("resize", () => {
    clearTimeout(timeout);
    timeout = setTimeout(() => set_sizes(), 100)
})


// Match font sizes (for tiles and headline) fit their containers.
const set_sizes = () => {
    // using body width because widnow width acts weird in dev/inspect mode
    const body_width = document.body.clientWidth;

    if (body_width < 500) {

        // resize title
        let new_tile_font_size = Math.round(body_width / 8.7).toString() + "px"
        board.style.fontSize = new_tile_font_size

        // resize headline
        let new_headline_font_size = Math.round(body_width / 5).toString() + "px"
        headline.style.fontSize = new_headline_font_size

        // remove margins from board panel
        board_panel.style.marginTop = "0px";

    } else {
        board.style.fontSize = "57px"
        headline.style.fontSize = "101px"

        // re-institute margins on board panel
        board_panel.style.marginTop = "10px";
    }
}

/* 
 * 
 * 
 * 
 * 
 * ========================
 * ========================
 * ========================
 * =====              =====
 * =====  game stuff  =====
 * =====              =====
 * ========================
 * ========================
 * ========================
 * 
 * 
 * current letter should either blink or be a different color
 * current letter turns NULL after finishing (but before pressing enter on) a current word
 * 
 *
 * 
 * 
 */



// This is how we set the colors for each letter
const guess_map = {
    words: [
        // guess 1
        {
            tiles: [
                new Tile(document.getElementById("1-1"), LetterState.CURRENT),
                new Tile(document.getElementById("1-2"), LetterState.CURRENT),
                new Tile(document.getElementById("1-3"), LetterState.CURRENT),
                new Tile(document.getElementById("1-4"), LetterState.CURRENT),
                new Tile(document.getElementById("1-5"), LetterState.CURRENT),
            ]
        },

        // guess 2
        {
            tiles: [
                new Tile(document.getElementById("2-1"), LetterState.CURRENT),
                new Tile(document.getElementById("2-2"), LetterState.CURRENT),
                new Tile(document.getElementById("2-3"), LetterState.CURRENT),
                new Tile(document.getElementById("2-4"), LetterState.CURRENT),
                new Tile(document.getElementById("2-5"), LetterState.CURRENT),
            ]
        },

        // guess 3
        {
            tiles: [
                new Tile(document.getElementById("3-1"), LetterState.CURRENT),
                new Tile(document.getElementById("3-2"), LetterState.CURRENT),
                new Tile(document.getElementById("3-3"), LetterState.CURRENT),
                new Tile(document.getElementById("3-4"), LetterState.CURRENT),
                new Tile(document.getElementById("3-5"), LetterState.CURRENT),
            ]
        },

        // guess 4
        {
            tiles: [
                new Tile(document.getElementById("4-1"), LetterState.CURRENT),
                new Tile(document.getElementById("4-2"), LetterState.CURRENT),
                new Tile(document.getElementById("4-3"), LetterState.CURRENT),
                new Tile(document.getElementById("4-4"), LetterState.CURRENT),
                new Tile(document.getElementById("4-5"), LetterState.CURRENT),
            ]
        },

        // guess 5
        {
            tiles: [
                new Tile(document.getElementById("5-1"), LetterState.CURRENT),
                new Tile(document.getElementById("5-2"), LetterState.CURRENT),
                new Tile(document.getElementById("5-3"), LetterState.CURRENT),
                new Tile(document.getElementById("5-4"), LetterState.CURRENT),
                new Tile(document.getElementById("5-5"), LetterState.CURRENT),
            ]
        },
    ]
}

let word_index = 0
let letter_index = 0

let current_word = guess_map.words[word_index]
let current_tile = current_word.tiles[letter_index]



/*
 * Make sure current word is really full. No empty spaces.
 * TO DO: check it against list of actual words (much later)
 */
const current_word_is_ready = () => {
    let ready = true
    current_word.tiles.map(tile => {
        if (tile.letter == "" || !tile.letter) {
            console.log("word not ready")
            ready = false
        }
    })

    return ready
}

/**
 * After every line we check the input word against the winning word.
 * Do multiple runs to give precedence to right_spot.
 */
const check_word = async () => {
    // Make sure word is ready
    if (!current_word_is_ready()) {
        new_message("Please finish the word")
        return
    }

    // Make word from chars
    const full_word = current_word.tiles.reduce((str, tile) => str + tile.letter, "")
    const letter_states_obj = await io.check_word_io(full_word)

    // Show Error
    if (!!letter_states_obj.error) {
        new_message(letter_states_obj.error)
        return
    } else if (
        !letter_states_obj.letter_states ||
        !Array.isArray(letter_states_obj.letter_states) ||
        letter_states_obj.letter_states.length != current_word.tiles.length
    ) {
        new_message("Bad server response")
        return
    }

    let full_word_correct = true

    // map result onto tiles
    letter_states_obj.letter_states.map((letter_state, index) => {
        const tile = current_word.tiles[index]
        //current_word.tiles[index].element.classList.add(letter_state)
        tile.state = letter_state  
        tile.element.classList.remove(LetterState.CURRENT)  
        tile.element.classList.add(tile.state)  
        if (letter_state != LetterState.RIGHT_SPOT) {
            full_word_correct = false
        }
    })

    if (full_word_correct) {
        end_game(true, full_word)
        return
    }

    // move on to next guess
    letter_index = 0
    word_index ++

    if (word_index > 4) {
        end_game(false, full_word)
        return
    }

    current_word = guess_map.words[word_index]
    set_current_tile(current_word.tiles[letter_index])
    remove_tabindexes() // remove old (all) tabindexes
    set_tabindexes() // set NEW tabindexes
    current_tile.element.focus()
}


const end_game = (victory, word) => {
    const endgame_msg = "You " + 
        (victory ? "Win!" : "Lose!") +
        "<br/>The word was " +
        "<h3>" + word + "<h3>"
    current_word = null
    set_current_tile(null)
    unset_current_tile_classes()
    remove_tabindexes()
    new_message(endgame_msg)
}


/**
 * User pressed a key.
 * We accept letters, ENTER, and BACKSPACE.
 * letters go into the current letter tile (unless word is full and unckecked)
 * ENTER checks the word IF the word is full.
 * BACKSPACE deletes the previous letter in word.
 * @param {*} event 
*/
const key_pressed = event => {
    if (!current_tile || !current_word) {
        console.log("game is over, current things are NULL")
        return
    }
    
    const key = event.key.toString().toUpperCase()

    // Check for relevant non-letter keys first
    if (key == "ENTER") {
        check_word()
        return
    } else if (
        ( key === "TAB" && event.shiftKey ) ||
        key == "ARROWLEFT"
    ) {
        move_left(event)
        return
    } else if (key === "TAB" || key == "ARROWRIGHT") {
        move_right(event)
        return
    } else if (key === "BACKSPACE") {
        backspace()
        return
    }else if (key === "DELETE") {
        current_tile.set_letter("")
        return
    }

    /**
     * Filter out non-relevant non-letter keys.
     * regex for only letters, and only ONE letter.
     * also make sure the modal is not open
     */

    if (!/^[a-z]$/i.test(key) || document.getElementById('message_modal').style.display == "block" ) {
        return
    }

    current_tile.set_letter(key)
    current_tile.element.classList.add(LetterState.CURRENT)

    letter_index < 4 && letter_index++
    set_current_tile(current_word.tiles[letter_index])
    current_tile.element.focus()
}


/* 
 * 
 * 
 * ========================
 * ========================
 * =====              =====
 * =====  NAVIGATION  =====
 * =====              =====
 * ========================
 * ========================
 * 
 * 
*/


const move_left = (event = null) => {
    if (letter_index > 0 ) {
        letter_index--
        set_current_tile(current_word.tiles[letter_index])
        current_tile.element.focus()
        event && event.preventDefault()
    }
}


const move_right = (event = null) => {
    if (letter_index < 4) {
        letter_index++
        set_current_tile(current_word.tiles[letter_index])
        current_tile.element.focus()

        event && event.preventDefault()
    }
}

const backspace = (event = null) => {
    current_tile.set_letter("")

    if (letter_index > 0) {
        letter_index--
        set_current_tile(current_word.tiles[letter_index])
        current_tile.set_letter("")
        current_tile.element.focus()

        event && event.preventDefault()
    }
}



/* 
 * 
 * =============================
 * =============================
 * =====                   =====
 * =====  OTHER FUNCTIONS  =====
 * =====                   =====
 * =============================
 * =============================
 * 
*/



const start_game = () => {
    set_sizes()
    set_tabindexes()
    current_tile.element.focus()
    guess_map.words.map(word => {
        make_tiles_clickable(word.tiles)
    })
}


// Open the modal with incoming message
const new_message = text => {
    message_p.innerHTML = text
    message_modal.foundation('open');
}


/**
 * REMOVE FOCUS FROM MODAL ON MODAL-CLOSE
 */
$('#message_modal').on('closed.zf.reveal', () => {
    setTimeout(() => {
        if (current_tile.element) {
            current_tile.element.focus()
        }
    }, 1)
})


/**
 * Make tiles in current_word tabbable.
 * This is for accessibility but also provides
 * something to focus on after modal close.
 */
const set_tabindexes = () => {
    current_word.tiles.map(tile => {
        tile.element.setAttribute('tabindex', '0')
    })
}


/**
 * Remove tab indexes from ALL tiles.
 * This is called before adding tab indexes
 * on current_word ONLY.
 */
const remove_tabindexes = () => {
    guess_map.words.map(word => {
        word.tiles.map(tile => 
            tile.element.removeAttribute('tabindex')
        )
    })
}

/**
 * Add click listener to each tile in given array (word)
 * @param {*} tiles_array 
 */
const make_tiles_clickable = tiles_array => {
    tiles_array.map(tile =>
        tile.element.addEventListener("click", set_tile_event)
    )
}

/**
 * Tile is clicked, so make it current (if it's in the current_word)
 * @param {*} event 
 */
const set_tile_event = event => {
    if (current_word == null) { return }
    // get the clicked element's id from the click event
    const el_id = event.currentTarget.id
    // find the tile in the tiles array (only for current_word)
    current_word.tiles.map(tile => {
        if (tile.element.id == el_id) {
            set_current_tile(tile)

            const new_letter_index = parseInt(el_id.charAt(el_id.length - 1))
            if (!!new_letter_index) {
                letter_index = new_letter_index - 1
            }
        }
    })
}

/**
 * When current_tile is set, we must give it the right class,
 * and remove that class from the previous current_tile
 * @param {Tile} tile 
 */
const set_current_tile = tile => {
    if (tile == null) { return }
    unset_current_tile_classes()
    current_tile = tile
    current_tile.element.classList.add("current_tile")
}

const unset_current_tile_classes = () => {
    guess_map.words.map(word =>
        word.tiles.map(tile =>
            tile.element.classList.remove("current_tile")
        )
    )
}


/**
 * TODO:
 * 
 * INCORPORATING BACKEND:
 * * API returns list of LetterStates (real word is never shared, obviously)
 * * Make this a MODULE (so we can import files and interact with API)
 * * List of accepted words
 * * LARGE list of words user can enter
 * * SMALLER list of words it actually might BE.
 * 
 * 
 * 
 * 
 * FIRST:
 * 1. Simultaneously check it in the backend AND the front end.
 * 2. Replace front-end check with backend check.
 * 
 */

window.check_word = check_word