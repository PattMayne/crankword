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
 * 
 * 
 * ----------------------------------
 * ----------------------------------
 * -----------            -----------
 * -----------  AUTH APP  -----------
 * -----------            -----------
 * ----------------------------------
 * ----------------------------------
 * 
 * 
 * 
 * 
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
    }).then(guess_map => {
        if (!!guess_map.fake_word) {
            console.log("FAKE WORD")
            response_obj.fake_word = true
        } else if (!!guess_map.max_guesses) {
            console.log("MAX GUESSES")
            response_obj.max_guesses = true
        } else {
            console.log("Guess Map: ", guess_map)
            response_obj.letter_states = guess_map
            console.log("return length 1: " + guess_map.length)
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    console.log("return length 2: " + response_obj.letter_states.length)
    return response_obj
}


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
            console.log("JOINED GAME")
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
            console.log("STARTED GAME")
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


export const get_guess_scores = async game_id => {
    const route = "/game_in/get_guess_scores"
    const input = {
        "game_id": parseInt(game_id)
    }

    const response_obj = {
        scores: [],
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

        console.log("RAW DATA: " + JSON.stringify(data))

        if (!!data.scores) {
            console.log("got the scores")
            response_obj.scores = data.scores
        } else {
            console.log("DID NOT GET GUESS DATA")
            response_obj.error = !!data.error ? data.error : "DID NOT GET GUESS DATA"
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })

    return response_obj
}