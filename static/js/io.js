import * as utils from './utils.js'

/* 
 * 
 * ---------------------------------
 * ----------             ----------
 * ----------  IO ROUTES  ----------
 * ----------             ----------
 * ---------------------------------
 * 
 * 
 * 
*/


/* 
 * 
 * 
 * 
 * 
 * -------------------------------
 * -------------------------------
 * -----------         -----------
 * -----------  LOCAL  -----------
 * -----------         -----------
 * -------------------------------
 * -------------------------------
 * 
 * 
 * "local" meaning the crankword APIs
 * instead of the auth_app APIs
 * 
 * 
 * 
*/

/**
 * User presses "new game" button.
 * We call the "new game" function in the backend.
 * Backend creates an empty new game and returns id.
 * @returns 
 */
export const new_game = async () => {
    const route = "/new_game"

    let response = await fetch(route, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' }
    })

    let data = await response.json()

    let return_obj = {
        game_id: 0,
        error: null
    }

    if (data.game_id !== undefined) {
        return_obj.game_id = data.game_id
    } else if (!!data.error) {
        return_obj.error = data.error
    } else {
        console.log("no game id")
        return_obj.error = "NO GAME ID"
    }

    return return_obj
}


/**
 * When the user wants to join the game.
 * 
 * @param {int} game_id 
 * @returns obj
 */
export const join_game = async (game_id) => {
    const route = "/game_in/join_game"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        success: false,
        error: null
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to join game, or error on server.")
        }
        return response.json()
    }).then(data => {
        if (data.success) {
            response_obj.success = true
        } else {
            console.log("DID NOT JOIN GAME")
            response_obj.error = !!data.error ? data.error : "DID NOT JOIN GAME"
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}

/**
 * When the owner of the game wants to transition from pre-game to in-progress.
 * 
 * @param {int} game_id 
 * @returns json object
 */
export const start_game = async game_id => {
    const route = "/game_in/start_game"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        success: false,
        error: null
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to start game, or error on server.")
        }
        return response.json()
    }).then(data => {
        if (data.success) {
            response_obj.success = true
        } else {
            console.log("DID NOT START GAME")
            response_obj.error = !!data.error ? data.error : "DID NOT START GAME"
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}


/**
 * update the data about the pregame-status game.
 * 
 * @param {int} game_id 
 * @returns obj
 */
export const refresh_pregame = async game_id => {
    const route = "/game_in/refresh_pregame"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        players: [],
        game_status: "in_progress"
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to refresh game, or error on server.")
        }
        return response.json()
    }).then(data => {
        if (!!data.game_status && !!data.players) {
            response_obj.players = data.players
            response_obj.game_status = data.game_status
        } else {
            console.log("DID NOT REFRESH GAME DATA")
            response_obj.error = !!data.error ? data.error : "DID NOT REFRESH GAME DATA"
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}


/**
 * Get all of the current player's previous guesses and their scores
 * from the database.
 * 
 * @param {int} game_id 
 * @returns obj
 */
export const get_guess_scores = async game_id => {
    const route = "/game_in/get_guess_scores"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        scores: null,
        error: null
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to get guesses, or error on server.")
        }
        return response.json()
    }).then(data => {

        if (!!data.scores) {
            response_obj.scores = data.scores
        } else {
            console.log("DID NOT GET GUESS DATA")
            response_obj.error = !!data.error ? data.error : "DID NOT GET GUESS DATA"
        }        
    }).catch(error =>
        console.log('Error: ', error)
    )

    return response_obj
}

/**
 * Check a particular guess (word) and get a result for that word
 * @param {*} guess_word 
 * @param {*} game_id 
 * @returns 
 */
export const check_guess_io = async (guess_word, game_id) => {
    const check_guess_route = "/game_in/check_guess"
    const guess_obj = {
        "guess_word": guess_word,
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        letter_states: [],
        fake_word: false,
        max_guesses: false,
        wrong_turn: false,
        game_over: false,
        is_winner: false,
        error: null
    }

    await utils.fetch_json_post(check_guess_route, guess_obj)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to check word, or error on server.")
        }
        return response.json()
    }).then(guess_result => {
        if (!!guess_result.fake_word) {
            response_obj.fake_word = true
        } else if (!!guess_result.max_guesses) {
            response_obj.max_guesses = true
        } else if (!!guess_result.wrong_turn) {
            response_obj.wrong_turn = true
        } else {
            if (guess_result.is_winner) {
                response_obj.is_winner = guess_result.is_winner
                response_obj.game_over = true
            } else if (guess_result.game_over) {
                response_obj.game_over = true
            }
            
            response_obj.letter_states = guess_result.score
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}


/**
 * update the data about the pregame-status game.
 * 
 * @param {int} game_id 
 * @returns obj
 */
export const refresh_players = async game_id => {
    const route = "/game_in/refresh_in_prog_players"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        players: [],
        current_turn_id: null,
        game_over: false
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to refresh game, or error on server.")
        }
        return response.json()
    }).then(data => {
        if (!!data.current_turn_id && !!data.players) {
            response_obj.players = data.players
            response_obj.current_turn_id = data.current_turn_id
            response_obj.game_over = !!data.game_over
        } else {
            console.log("DID NOT REFRESH PLAYERS DATA")
            response_obj.error = !!data.error ? data.error : "DID NOT REFRESH PLAYERS DATA"
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}