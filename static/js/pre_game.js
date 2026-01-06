$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []

const join_game = async () => {
    console.log("JOINING GAME")
    msgs = []
    msgs.push("NEW GAME CREATING")

    let game_id = document.getElementById("game_id").value
    let join_response = await io.join_game(game_id)

    if (!join_response.success || !!join_response.error) {
        console.log(JSON.stringify(join_response))
        console.log("error")
        msgs = []
        msgs.push(join_response.error)
        show_msg_box()
        return
    }

    console.log("no error?")

    // reload
    window.location.reload()
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

document.addEventListener('DOMContentLoaded', () => {
    hide_msg_box()

    const join_btn = document.getElementById('join_btn')

    if (join_btn) {
        join_btn.addEventListener(
            'click', (e) => join_game())
    }

})


window.join_game = join_game