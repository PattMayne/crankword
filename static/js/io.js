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

export const check_word_io = async guess_word => {
    const check_word_route = "/game_in/check_word"
    const guess_obj = {
        "guess_word": guess_word
    }

    const response_obj = {
        letter_states: [],
        error: null
    }

    await utils.fetch_json_post(check_word_route, guess_obj)
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

export const new_game = async () => {
    const route = "/new_game"

    let response = await fetch(route, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' }
    })

    console.log("response: " + JSON.stringify(response))

    return response
}