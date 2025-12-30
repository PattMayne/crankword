$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []

const create_new_game = async () => {
    console.log("CREATING NEW GAME")
    msgs = []
    msgs.push("NEW GAME CREATING")

    let game_id = await io.new_game()

    if (!game_id || game_id < 1) {
        msgs = []
        msgs.push("ERROR CREATING NEW GAME")
        show_msg_box()
        return
    }

    // Redirect user to game
    const game_uri = "/game/" + game_id
    window.location.href = game_uri
}

// SHOW/HIDE ERROR BOX

const hide_msg_box = () =>
    document.getElementById("msg_box").style.display = "none"

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";

    for (let msg of msgs) {
        const msg_p = "<p>" + msg + "</p>"
        msg_box.innerHTML += msg_p
    }

    msg_box.style.display = ""
}


// Add event listeners

document.addEventListener('DOMContentLoaded', () => hide_msg_box())

document.getElementById('new_game_button').addEventListener(
    'click', (e) => create_new_game())


window.create_new_game = create_new_game