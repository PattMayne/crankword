import * as utils from './utils.js'
/* 
 * 
 * ---------------------------------
 * ----------             ----------
 * ----------  IO ROUTES  ----------
 * ----------             ----------
 * ---------------------------------
 * 
*/


export const check_word_io = async guess_word => {
    const check_word_route = "/game_in/check_word"
    const guess_obj = {
        "guess_word": guess_word
    }

    await utils.fetch_json_post(check_word_route, guess_obj)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                // err_msgs.push(msg)
                // show_err_box()
            })

            throw new Error("Unable to check word, or error on server.")
        }
        return response.json()
    }).then(guess_map => {
        console.log("Guess Map: ", guess_map)
    }).catch(error => {
        console.log('Error: ', error)
    })
}
