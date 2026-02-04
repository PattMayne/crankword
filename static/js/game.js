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


const board_panel = document.getElementById("board_panel")
const board = document.getElementById("board")
const headline = document.getElementById("headline")
const message_modal = $('#message_modal') // Foundation demands jquery for this
const cancel_modal = $('#cancel_modal')
const message_p = document.getElementById("message_p")

let showing_scores = true

// Game id is in the path (game/id)
const hashed_game_id = () => document.getElementById("hashed_game_id").value

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
    const panel_width = document.getElementById("board_panel").offsetWidth
    document.getElementById("game_menu_container").style.width = "" + panel_width + "px"

    if (panel_width < 500) {

        // resize title
        let new_tile_font_size = Math.round(panel_width / 8.7).toString() + "px"
        board.style.fontSize = new_tile_font_size

        // resize headline
        let new_headline_font_size = Math.round(panel_width / 5).toString() + "px"
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

let game_over = false
let turn_timeout = null
let timer_element = null
let current_turn_id = null
let username = null
let user_id = null
let number_of_players = 0

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
    const letter_states_obj = await io.check_guess_io(full_word, hashed_game_id())
    current_turn_id = letter_states_obj.next_turn_id

    //console.log("letter_states_obj::: " + JSON.stringify(letter_states_obj))

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
        tile.state = letter_state  
        tile.element.classList.remove(LetterState.CURRENT)  
        tile.element.classList.add(tile.state)  
        if (letter_state != LetterState.RIGHT_SPOT) {
            full_word_correct = false
        }

        update_keyboard_letter(tile.letter, letter_state)
    })

    create_keyboard_element()

    if (letter_states_obj.game_over) {
        end_game(letter_states_obj.is_winner)
        return
    }

    // move on to next guess
    letter_index = 0
    word_index ++

    if (word_index < 5) {        
        current_word = guess_map.words[word_index]
        set_current_tile(current_word.tiles[letter_index])
        remove_tabindexes() // remove old (all) tabindexes
        set_tabindexes() // set NEW tabindexes
        current_tile.element.focus()
    }

    remove_tabindexes() // remove old (all) tabindexes
    refresh_players()
}


const end_game = (victory) => {
    if (game_over) {
        return
    }

    game_over = true
    const endgame_msg = "You " + 
        (victory ? "Win!" : "Lose!")
    current_word = null
    set_current_tile(null)
    unset_current_tile_classes()
    remove_tabindexes()
    new_message(endgame_msg)

    // Give player time to see result, then reload to FinishedGame page
    setTimeout(() => {
        location.reload();
    }, 2000)
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

$('#cancel_modal').on('closed.zf.reveal', () => {
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
    const scores_obj = await io.get_guess_scores(hashed_game_id())

    // set the results into the grid

    word_index = 0
    letter_index = 0
    current_word = guess_map.words[word_index]
    current_tile = current_word.tiles[letter_index]

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

            // update KEYBOARD_LETTERS
            update_keyboard_letter(guess.word[k], guess.score[k])
        }
    }

    create_keyboard_element()

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
    }
}


const refresh_players = async () => {
    const players_obj = await io.refresh_players(hashed_game_id())
    let current_player_name = ""

    if (
        !players_obj.current_turn_id ||
        !players_obj.players ||
        players_obj.players.length == 0
    ) {
        console.log("MISSING PLAYERS DATA")
        return
    } else if (players_obj.game_over) {
        end_game(false)
    }

    // we need to know if this is a single-player game
    number_of_players = players_obj.players.length
    
    // Check if it's player's turn, and if the turn has changed through timeout
    if (current_turn_id != players_obj.current_turn_id) {
        if (current_turn_id != null && current_turn_id == user_id){
            new_message("MISSED YOUR TURN BY TIMEOUT!")
            settle_old_scores()
        }

        current_turn_id = players_obj.current_turn_id
    }


    // We have the players.
    // They're already sorted, but we have to start with the current turn id.
    // Do two loops, first to catch the current player and their following players,
    // second to get any preceding players missed on the first loop.

    const players = []
    let current_turn_found = false

    // Get current turn player and following players
    players_obj.players.map(player_obj => {
        if (player_obj.user_id == players_obj.current_turn_id) {
            current_turn_found = true
            players.push(player_obj)
            current_player_name = player_obj.username
        } else if (current_turn_found) {
            players.push(player_obj)
        }
    })


    current_turn_found = false
    // 2nd loop: get the usernames preceding the current_turn_id
    players_obj.players.map(player_obj => {
        if (player_obj.user_id == players_obj.current_turn_id) {
            current_turn_found = true
        } else if (!current_turn_found) {
            players.push(player_obj)
        }
    })

    // players is now arranged properly.
    // Set the names in the list.

    let players_html = ""

    players.map((player, index) => {

        if (index == 0) {
            const player_turn_li_element = document.getElementById("player_turn_li")
            player_turn_li_element.innerHTML = player.username
        } else {
            players_html += build_player_li(player.username)
        }

    })

    const players_list_element = document.getElementById("players_list")
    players_list_element.innerHTML = players_html

    show_oppo_scores(players)

    // now do the timer

    if (!!players_obj.turn_timeout) {
        turn_timeout = players_obj.turn_timeout
    }
}

const build_player_li = username => "<li " +
    "class='player_label'" +
    ">&nbsp;" + username + "</li>"


const increment_turn_countdown = () => {
    if (turn_timeout !=  null) {
        const now = new Date()
        const diff_in_ms = turn_timeout - now
        const diff_in_seconds = Math.floor(diff_in_ms / 1000)
        timer_element.innerHTML = diff_in_seconds
    }
}

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

/**
 * When we get updated data about the players' scores,
 * this function represents those scores on little cards.
 * The canvases are coded into HTML from the template,
 * but this function draws the scores onto the canvases.
 * @param {list} players 
 */
const show_oppo_scores = (players) => {
    // We expect each player to have an oppo_panel containing an oppo_panel and oppo_canvas
    // For each oppo_panel we set their scores in the squares

    let rect_width = null // fill on first loop, then read from filled variable
    const max_words = 5

    players.map(player => {
        const label_id = player.username + "_label"
        const canvas_id = player.username + "_canvas"

        const oppo_label = document.getElementById(label_id)
        oppo_label.innerHTML = player.username

        const oppo_canvas = document.getElementById(canvas_id)
        const word_scores = player.scores

        const context = oppo_canvas.getContext("2d")
        context.clearRect(0, 0, oppo_canvas.width, oppo_canvas.height)
        
        // draw the scores
        rect_width = (rect_width != null) ?
            rect_width :
            (oppo_canvas.width -
                ((OPPO_SCORE_DATA.rects_per_line + 1) * OPPO_SCORE_DATA.border_size)) /
                OPPO_SCORE_DATA.rects_per_line
        
        for (let i=0; i<word_scores.length; i++) {
            const letter_scores = word_scores[i].score

            for (let k=0; k<letter_scores.length; k++) {
                const letter_score = letter_scores[k]
                const x = OPPO_SCORE_DATA.border_size + (k * (OPPO_SCORE_DATA.border_size + rect_width))
                const y = OPPO_SCORE_DATA.border_size + (i * (OPPO_SCORE_DATA.border_size + rect_width))
                context.fillStyle = OPPO_SCORE_DATA.COLORS[letter_score]
                context.fillRect(x, y, rect_width, rect_width)
            }
        }

        const index_start =
            word_scores.length > 0 ?
            word_scores.length - 1 :
            0

        for (let i=index_start; i<max_words; i++) {
            for (let k=0; k<OPPO_SCORE_DATA.rects_per_line; k++) {
                const x = OPPO_SCORE_DATA.border_size + (k * (OPPO_SCORE_DATA.border_size + rect_width))
                const y = OPPO_SCORE_DATA.border_size + (i * (OPPO_SCORE_DATA.border_size + rect_width))
                context.strokeStyle = OPPO_SCORE_DATA.COLORS["dud"]
                context.strokeRect(x, y, rect_width, rect_width)
            }
        }
    })
}

function toggle_scores() {
    if (!showing_scores) {
        show_scores()
    } else {
        hide_scores()
    }
}

function show_scores() {
    showing_scores = true
    document.getElementById("oppo_scores").style.display = ""
    document.getElementById("scores_toggle").innerHTML = "HIDE PANEL"
    document.getElementById("scores_toggle_2").innerHTML = "HIDE PANEL"
    document.getElementById("crank_cell").className = "large-7 medium-12 small-12 cell"
    document.getElementById("stats_cell").style.display = ""
}

function hide_scores() {
    showing_scores = false
    document.getElementById("oppo_scores").style.display = "none"
    document.getElementById("scores_toggle").innerHTML = "SHOW PANEL"
    document.getElementById("scores_toggle_2").innerHTML = "SHOW PANEL"
    document.getElementById("crank_cell").className = "large-12 cell"
    document.getElementById("stats_cell").style.display = "none"
}




/* 
 * 
 * 
 * 
 * 
 * =============================
 * =============================
 * =====                   =====
 * =====  VISUAL KEYBOARD  =====
 * =====                   =====
 * =============================
 * =============================
 * 
 * 
 * 
 * 
*/


const KEYBOARD_LETTERS = {
    "q": false,
    "w": false,
    "e": false,
    "r": false,
    "t": false,
    "y": false,
    "q": false,
    "u": false,
    "i": false,
    "o": false,
    "p": false,
    "a": false,
    "s": false,
    "d": false,
    "f": false,
    "g": false,
    "h": false,
    "j": false,
    "k": false,
    "l": false,
    "z": false,
    "x": false,
    "c": false,
    "v": false,
    "b": false,
    "n": false,
    "m": false
}

// Update an individual letter in the keyboard which displays
// which letters have been used.
const update_keyboard_letter = (letter, score) => {
    letter = letter.toLowerCase()

    // do not override "right spot" status
    if (KEYBOARD_LETTERS[letter] != LetterState.RIGHT_SPOT) {
        KEYBOARD_LETTERS[letter] = score
    }
}


// Create the keyboard element which displays
// which letters have been used.
const create_keyboard_element = () => {
    // get the element where we will draw the keyboard
    const used_keys_board = document.getElementById("used_keys_board_container")


    let html = ""
    const first_row_starts = 0
    const second_row_starts = 10
    const third_row_starts = 19

    let key_count = 0

    Object.entries(KEYBOARD_LETTERS).forEach(([key, value]) => {
        if (key_count == first_row_starts) {
            html += "<div class='keyboard_row'>"
        } else if (key_count == second_row_starts || key_count == third_row_starts) {
            html += "</div><div class='keyboard_row'>"
        }

        html += create_keyboard_letter_div(key, value)
        key_count ++

        if (key_count >= Object.keys(KEYBOARD_LETTERS).length) {
            html += "</div>"
        }
    })

    // enter and backspace keys
    html += generate_enter_backspace()
    used_keys_board.innerHTML = html
    add_virtual_keyboard_listeners()
}


// The elements exist. Now add event listeners to each one.
const add_virtual_keyboard_listeners = () => {
    const kb_letters = document.querySelectorAll(".kb_letter")
    const back_key = document.getElementById("back_key")
    const enter_key = document.getElementById("enter_key")
    
    kb_letters.forEach(kb_letter => {
        kb_letter.addEventListener("click", () => {
            const event = { key: kb_letter.innerHTML }
            key_pressed(event)
        })
    })

    back_key.addEventListener("click", () => {
        const event = { key: "BACKSPACE" }
        key_pressed(event)
    })

    enter_key.addEventListener("click", () => {
        const event = { key: "ENTER" }
        key_pressed(event)
    })
}


// create the "enter" and "back" keys for virtual keyboard
// return as HTML string
const generate_enter_backspace = () => {
    let html = ""
    html += "<div class='keyboard_row'>"
    html += "<div id='back_key'>BACK</div>"
    html += "<div id='enter_key'>ENTER</div>"
    html += "</div>"
    return html
}


// create the individual div for an individual letter.
const create_keyboard_letter_div = (letter, state) => {
    let html = state == false ? "<div class='kb_letter letter_not_used'>" :
        state == LetterState.RIGHT_SPOT ? "<div class='kb_letter letter_right_spot'>" :
        state == LetterState.WRONG_SPOT ? "<div class='kb_letter letter_wrong_spot'>" :
        state == LetterState.DUD ? "<div class='kb_letter letter_dud'>" :
        "<div>"
    html += letter.toUpperCase() + "</div>"

    return html
}


// SETUP STUFF

document.addEventListener('DOMContentLoaded', async () => {
    timer_element = document.getElementById("timer_element")
    const cancel_button = document.getElementById("cancel_button")
    const confirm_cancel_button = document.getElementById("confirm_cancel_button")
    document.getElementById('scores_toggle').addEventListener('click', toggle_scores)
    document.getElementById('scores_toggle_2').addEventListener('click', toggle_scores)
    !!cancel_button && cancel_button.addEventListener('click', () => {
        cancel_modal.foundation('open')
    })
    !!confirm_cancel_button && confirm_cancel_button.addEventListener('click', cancel_game)
    username = document.getElementById("username").value
    user_id = document.getElementById("user_id").value
    settle_old_scores()
    await refresh_players()


    if (number_of_players > 1) {
        // Check every 1.5 seconds for new users or updated game_status
        setInterval(refresh_players, 1500)
        setInterval(increment_turn_countdown, 1000)
    } else {
        // if single-player game, the countdown is null
        timer_element.innerHTML = "--"
        // Check every 5 seconds for new users or updated game_status
        setInterval(refresh_players, 5000)
    }
    show_scores()
})

const cancel_game = async () => {
    const cancel_response = await io.cancel_game(hashed_game_id())

    if (cancel_response.success) {
        window.location.reload()
    } else {
        console.log("errrrorrrr")
    }
}

window.check_guess = check_guess