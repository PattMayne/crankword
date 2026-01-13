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

// Game id is in the path (game/id)
const game_id = () => {
    if (!!game_id_storage) {
        return game_id_storage
    }

    const path = window.location.pathname
    const parts = path.split('/')
    return parts.length > 0 ? parts[parts.length - 1] : null
}

let game_id_storage = null

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
const current_guess_is_ready = () => {
    let is_ready = true
    current_word.tiles.map(tile => {
        if (tile.letter == "" || !tile.letter) {
            console.log("word not ready")
            is_ready = false
        }
    })

    return is_ready
}

/**
 * After every line we check the input word against the winning word.
 * Do multiple runs to give precedence to right_spot.
 */
const check_guess = async () => {
    // Make sure word is ready
    if (!current_guess_is_ready()) {
        new_message("Please finish the word")
        return
    }

    // Make word from chars
    const full_word = current_word.tiles.reduce((str, tile) => str + tile.letter, "")
    const letter_states_obj = await io.check_guess_io(full_word, game_id())

    // Show Error
    if (!!letter_states_obj.error) {
        console.log("ERROR")
        new_message(letter_states_obj.error)
        return
    } else if (letter_states_obj.fake_word) {
        new_message("NOT IN WORD LIST")
        letter_index = 0
        set_current_tile(current_word.tiles[letter_index])
        current_tile.element.focus()
        return
    } else if (letter_states_obj.max_guesses) {
        new_message("NO MORE GUESSES")
        letter_index = 0
        set_current_tile(current_word.tiles[letter_index])
        current_tile.element.focus()
        return
    } else if (letter_states_obj.wrong_turn) {
        new_message("NOT YOUR TURN")
        letter_index = 0
        set_current_tile(current_word.tiles[letter_index])
        current_tile.element.focus()
        return
    } else if (
        !letter_states_obj.letter_states ||
        !Array.isArray(letter_states_obj.letter_states) ||
        letter_states_obj.letter_states.length != current_word.tiles.length
    ) {
        new_message("Bad server response")
        return
    }

    // make win TRUE and then prove it FALSE
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

    refresh_players()
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
        check_guess()
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
    current_tile.element.classList.add("current_tile")
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

const settle_old_scores = async () => {

    // get the list of guess results
    const scores_obj = await io.get_guess_scores(game_id())

    // set the results into the grid

    word_index = 0
    letter_index = 0
    current_word = guess_map.words[word_index]
    current_tile = current_word.tiles[letter_index]

    //current_tile.set_letter(key)

    for (let i=0; i<scores_obj.scores.length; i++) {
        const guess = scores_obj.scores[i]
        word_index = i
        current_word = guess_map.words[word_index]

        for (let k=0; k<guess.score.length; k++) {
            letter_index = k
            current_tile = current_word.tiles[letter_index]
            current_tile.set_letter(guess.word[k])
            current_tile.element.classList.remove(LetterState.CURRENT)
            current_tile.state = guess.score[k]
            current_tile.element.classList.add(current_tile.state)
        }
    }

    if (word_index > 3) {
        // If all guesses are full, remove all interactiveness
        current_word = null
        set_current_tile(null)
        unset_current_tile_classes()
        remove_tabindexes()
    } else if (scores_obj.scores.length > 0) {
        // If we set ANY guesses (length > 0), remove their interactivity,
        // but also set the NEXT word as the current word.
        unset_current_tile_classes()
        remove_tabindexes()
        word_index++
        letter_index = 0
        current_word = guess_map.words[word_index]
        current_tile = current_word.tiles[letter_index]
        current_tile.element.classList.add("current_tile")
        current_tile.element.classList.add("current_guess")
        set_tabindexes()
        current_tile.element.focus()

        // TESTING DESIGN
        const test_scores = []
        scores_obj.scores.map(this_score => {
            test_scores.push(this_score.score)
        })

        show_oppo_scores("ok", test_scores)
    }
}


const refresh_players = async () => {
    const players_obj = await io.refresh_players(game_id())

    console.log("PLAYERS DATA: " + JSON.stringify(players_obj))

    if (
        !players_obj.current_turn_id ||
        !players_obj.players ||
        players_obj.players.length == 0
    ) {
        console.log("MISSING PLAYERS DATA")
        return
    }
    

    // We have the players.
    // They're already sorted, but we have to start with the current turn id.
    // Do two loops, first to catch the current player and their following players,
    // second to get any preceding players missed on the first loop.

    const player_names = []
    let current_turn_found = false
    let current_player_name = ""

    // Get current turn player and following players
    players_obj.players.map(player_obj => {
        if (player_obj.user_id == players_obj.current_turn_id) {
            current_turn_found = true
            player_names.push(player_obj.username)
            current_player_name = player_obj.username
        } else if (current_turn_found) {
            player_names.push(player_obj.username)
        }
    })


    current_turn_found = false
    // 2nd loop: get the usernames preceding the current_turn_id
    players_obj.players.map(player_obj => {
        if (player_obj.user_id == players_obj.current_turn_id) {
            current_turn_found = true
        } else if (!current_turn_found) {
            player_names.push(player_obj.username)
        }
    })

    // player_names is now arranged properly.
    // Set the names in the list.

    let players_html = ""

    player_names.map((name, index) => {

        if (index == 0) {
            const player_turn_li_element = document.getElementById("player_turn_li")
            player_turn_li_element.innerHTML = name
        } else {
            players_html += build_player_li(name)
        }

    })

    const players_list_element = document.getElementById("players_list")
    players_list_element.innerHTML = players_html
}

const build_player_li = username => "<li " +
    "class='player_label'" +
    ">" + username + "</li>"


/**
 * TODO:
 * 
 * SHOW OPPONENT SCORES
 * END GAME UPON CORRECT ANSWER OR ALL PLAYERS FAIL
 * TIMER
 * 
 * 
 * 
 */


/**
 * DISPLAY OPPONENT SCORES
*/

const OPPO_SCORE_DATA = {
    COLORS: {
        "dud": "#111111",
        "wrong_spot": "#0008dd",
        "right_spot": "#60f000",
        "future": "transparent"
    },
    border_size: 5,
    rects_per_line: 5
}

const show_oppo_scores = (username, scores) => {
    // We will create the entire div here (with username h3),
    // BUT FOR NOW just draw the 25 squares

    const player_canvas = document.getElementById("player_canvas")
    const context = player_canvas.getContext("2d")
    console.log("canvas size: " + player_canvas.width + ", " + player_canvas.height)
    
    // draw the scores
    const rect_width = (player_canvas.width -
        ((OPPO_SCORE_DATA.rects_per_line + 1) * OPPO_SCORE_DATA.border_size))
         / OPPO_SCORE_DATA.rects_per_line

    // TODO: put this into maps instead
    for (let i=0; i<scores.length; i++) {
        const word = scores[i]

        for (let k=0; k<word.length; k++) {
            const score = word[k]
            const x = OPPO_SCORE_DATA.border_size + (k * (OPPO_SCORE_DATA.border_size + rect_width))
            const y = OPPO_SCORE_DATA.border_size + (i * (OPPO_SCORE_DATA.border_size + rect_width))
            context.fillStyle = OPPO_SCORE_DATA.COLORS[score]
            context.fillRect(x, y, rect_width, rect_width)
        }
    }
}




document.addEventListener('DOMContentLoaded', () => {
    game_id_storage = document.getElementById("game_id").value
    console.log("game id: " + game_id())
    settle_old_scores()
    refresh_players()

    // Check every 3 seconds for new users or updated game_status
    //setInterval(refresh_players, 3000);
})


window.check_guess = check_guess
