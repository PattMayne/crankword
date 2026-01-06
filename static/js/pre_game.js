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

const start_game = async () => {
    console.log("PRESSED 'START GAME'")
    msgs = []
    msgs.push("STARTING GAME...")

    const game_id = document.getElementById("game_id").value
    const start_response = await io.start_game(game_id)

    if (start_response.success) {
        window.location.reload()
    } else {
        console.log("errrrorrrr")
    }
}


const refresh_data = async () => {
    msgs = []
    msgs.push("REFRESHING GAME...")

    const game_id = document.getElementById("game_id").value
    const refresh_response = await io.refresh(game_id)

    if (!!refresh_response.game_status && !!refresh_response.players) {
        if (refresh_response.game_status != "pre_game") {
            window.location.reload()
            return
        }
        // else:
        set_players_list(refresh_response.players)        
    } else {
        console.log("errrrorrrr")
    }
}

const set_players_list = players_list => {

    const players_ul = document.getElementById("players_ul")
    let list_html = ""

    players_list.map(player_item => {
        if (!!player_item.username) {
            list_html += "<li>" + player_item.username + "</li>"
        }
    })
    
    players_ul.innerHTML = list_html
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


document.addEventListener('DOMContentLoaded', () => {
    hide_msg_box()
    
    // Add event listeners

    const join_btn = document.getElementById('join_btn')
    const start_btn = document.getElementById('start_btn')
    const refresh_btn = document.getElementById('refresh_btn')

    if (join_btn) {
        join_btn.addEventListener(
            'click', (e) => join_game())
    }

    if (start_btn) {
        start_btn.addEventListener(
            'click', (e) => start_game())
    }

    if (refresh_btn) {
        refresh_btn.addEventListener(
            'click', (e) => refresh_data())
    }

    // Check every 3 seconds for new users or updated game_status
    setInterval(refresh_data, 3000);
})

window.join_game = join_game
window.start_game = start_game
window.refresh_data = refresh_data