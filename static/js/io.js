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
        error: null
    }

    await utils.fetch_json_post(check_guess_route, guess_obj)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = error
            })

            throw new Error("Unable to check word, or error on server.")
        }
        return response.json()
    }).then(guess_map => {
        console.log("Guess Map: ", guess_map)
        response_obj.letter_states = guess_map
        console.log("return length 1: " + guess_map.length)
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

    if (data.game_id !== undefined) {
        return data.game_id
    } else {
        console.log("no game id")
        return null
    }
}