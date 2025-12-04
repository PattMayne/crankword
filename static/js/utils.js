

// Make sure the input string is within a given length range
const string_in_range = (range_obj, string) =>
    string.length >= range_obj.min && string.length <= range_obj.max




/**
 * For any time we are using fetch to send a JSON object to a POST API.
 * @param {String} route 
 * @param {JSON object} json_obj 
 * @returns HTTP response from a fetch call
 */
export const fetch_json_post = async (route, json_obj) => {
    // First create a JSON string, doing checks to ensure the obj is legit.
    const json_string =
        (typeof json_obj === "object" && json_obj !== null)
            ? (() => {
                try {
                    return JSON.stringify(json_obj)
                } catch {
                    return json_simple_error_string()
                }
            })() // the () immediately invokes the function I just defined
        : (typeof json_obj === "string" && is_valid_json_string(json_obj))
            ? json_obj
            : json_simple_error_string()

    // now we return the HTTP response from a fetch call
    return fetch(route, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: json_string
    })
}


/* make sure that a string is a legit JSON string which can be parsed. */ 
const is_valid_json_string = (json_string) => {
    try {
        return JSON.parse(json_string)
    } catch {
        return false
    }
}

// In case we have an error parsing the JSON, notify of error
const json_simple_error_string = () => JSON.stringify({ "error": "JSON response error" })


/*
 * 
 * 
 * =================================
 * =================================
 * =====                       =====
 * =====  game data stuctures  =====
 * =====                       =====
 * =================================
 * =================================
*/


// For which color to print it
export const LetterState = {
    CURRENT: "current_guess",
    RIGHT_SPOT: "right_spot",
    WRONG_SPOT: "wrong_spot",
    DUD: "dud"
}

// Data for the game tiles
export class Tile {
    constructor(element, state) {
        this.element = element
        this.state = state
        this.letter = ""
    }

    set_letter(letter) {
        this.letter = letter
        this.element.innerHTML = letter
        this.element.classList.add(LetterState.CURRENT)
    }
}

export const get_default_letter_states = number_of_letters => {
    const letter_states = []
    for (let i=0; i<number_of_letters; i++) {
        letter_states.push(LetterState.CURRENT)
    }

    return letter_states
}