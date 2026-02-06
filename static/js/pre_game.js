$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []
let number_of_players = 0

/**
 * When a user (non-owner) decides to "join" this game,
 * we call the API and get them in.
 */
const join_game = async () => {
    msgs = []

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

    // reload
    window.location.reload()
}

/**
 * When the owner presses the button to change game mode
 * from "pre_game" to "in_progress" (this starting the game)
 */
const start_game = async () => {
    msgs = []

    const game_id = document.getElementById("game_id").value
    const start_response = await io.start_game(game_id)

    if (start_response.success) {
        window.location.reload()
    } else {
        console.log("errrrorrrr")
    }
}

const cancel_game = async () => {
    msgs = []

    const game_id = document.getElementById("game_id").value
    const cancel_response = await io.cancel_game(game_id)

    if (cancel_response.success) {
        window.location.reload()
    } else {
        console.log("errrrorrrr")
    }
}


/**
 * Call the API to get updated information about this game,
 * then populate the relevant fields with that info.
 */
const refresh_data = async () => {
    const game_id = document.getElementById("game_id").value
    const refresh_response = await io.refresh_pregame(game_id)

    if (!!refresh_response.game_status && !!refresh_response.players) {

        if (refresh_response.game_status != "pre_game") {
            window.location.reload()
            return
        }

        // else:
        
        if (refresh_response.players.length < number_of_players) {
            msgs.push("Player was removed from game")
            show_msg_box()
        }

        number_of_players = refresh_response.players.length
        await set_players_list(refresh_response.players)
        await set_players_event_listeners(game_id, refresh_response.players)
        await set_pending_invites(refresh_response.invitee_usernames)   
        await set_invitee_event_listeners(game_id, refresh_response.invitee_usernames)
    } else {
        console.log("errrrorrrr")
    }
}

/**
 * Fill the players' list element with a list of the players
 * who have "joined" this game.
 * @param {array} players_list 
 */
const set_players_list = async players_list => {
    document.getElementById("players_ul").innerHTML =
        players_list.reduce((html, player_item) => 
            !!player_item.username ? 
                html + get_player_item_li(player_item.username) :
                html
        , "")
}

const get_player_item_li = username =>
    "<li>" +
    username +
    get_boot_btn(username) +
    "</li>"


const get_boot_btn = invitee_username =>
    "<a href='#' class='remove_player_button' id='" +
    get_boot_id(invitee_username) +
    "'>X</a>"



const set_players_event_listeners = async (game_id, player_items) => {
    player_items.map(player_item => {
        document.getElementById(get_boot_id(player_item.username))
            .addEventListener('click', (e) => {
                io.boot_player_pregame(game_id, player_item.username).then(result => {
                    msgs.push(result.message)
                    show_msg_box()
                    refresh_data()
                    msgs = []
                })
            })
    })
}

/**
 * Fill the relevant element with a list of the usernames who
 * have been invited to the game.
 * @param {array} invitee_usernames 
 */
const set_pending_invites = async invitee_usernames => {
    document.getElementById("invitees_ul").innerHTML =
        invitee_usernames.reduce((html, invitee_username) => {
            return html + "<li>" +
                invitee_username +
                get_delete_invite_btn(invitee_username) +
                "</li>"
        }, "")
}

const get_delete_invite_btn = invitee_username =>
    "<a href='#' class='remove_player_button' id='" +
    get_uninvite_id(invitee_username) +
    "'>X</a>"


const set_invitee_event_listeners = async (game_id, invitee_usernames) => {
    invitee_usernames.map(invitee_username => {
        document.getElementById(get_uninvite_id(invitee_username))
            .addEventListener('click', (e) => {
                io.uninvite_player(game_id, invitee_username).then(result => {
                    msgs.push(result.message)
                    show_msg_box()
                    refresh_data()
                    msgs = []
                })
            })
    })
}



const get_uninvite_id = username => "uninvite_" + username
const get_boot_id = username => "boot_" + username

/**
 * When the owner presses the button to invite another player
 */
const invite_player = async () => {
    const hash_game_id = document.getElementById("game_id").value
    const invited_username = document.getElementById("invite_input").value
    const invite_response = await io.invite_player(invited_username, hash_game_id)
    msgs.push(invite_response.message)
    show_msg_box()
    refresh_data()

    msgs = []
}

// SHOW/HIDE MESSAGE BOX

const hide_msg_box = () =>
    document.getElementById("msg_box").style.display = "none"

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";
    msgs.map(msg => msg_box.innerHTML += "<p>" + msg + "</p>" )
    msg_box.style.display = ""
}

const leave_game = async () => {
    console.log("Leaving game")

    msgs = []

    let game_id = document.getElementById("game_id").value
    let leave_resp = await io.leave_game(game_id)

    if (!leave_resp.success || !!leave_resp.error) {
        console.log(JSON.stringify(leave_resp))
        console.log("error")
        msgs = []
        msgs.push(leave_resp.error)
        show_msg_box()
        return
    }

    // reload
    window.location.reload()
}

document.addEventListener('DOMContentLoaded', () => {
    hide_msg_box()
    
    // get button elements so we can add event listeners
    const join_btn = document.getElementById('join_btn')
    const start_btn = document.getElementById('start_btn')
    const cancel_button = document.getElementById('cancel_btn')
    const refresh_btn = document.getElementById('refresh_btn')
    const invite_button = document.getElementById('invite_submit')
    const leave_btn = document.getElementById('leave_btn')
    const invite_input = document.getElementById('invite_input')

    // Add event listeners
    join_btn && join_btn.addEventListener('click', (e) => join_game())
    start_btn && start_btn.addEventListener('click', (e) => start_game())
    refresh_btn && refresh_btn.addEventListener('click', (e) => refresh_data())
    cancel_button && cancel_button.addEventListener('click', (e) => cancel_game())
    invite_button && invite_button.addEventListener('click', (e) => invite_player())
    leave_btn && leave_btn.addEventListener('click', (e) => leave_game())
    invite_input && invite_input.addEventListener("keydown", (event) => {
        const key = event.key.toString().toUpperCase()
        if (key == "ENTER") {
            event.preventDefault()
            invite_player()
        }
    });

    refresh_data()

    // Check every 3 seconds for new users or updated game_status
    setInterval(refresh_data, 3000);
})

window.join_game = join_game
window.start_game = start_game
window.cancel_game = cancel_game
window.refresh_data = refresh_data
window.invite_player = invite_player