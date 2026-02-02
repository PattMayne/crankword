$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []

/**
 * When a user (non-owner) decides to "join" this game,
 * we call the API and get them in.
 */
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

/**
 * When the owner presses the button to change game mode
 * from "pre_game" to "in_progress" (this starting the game)
 */
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
        set_players_list(refresh_response.players)
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


/**
 * Fill the relevant element with a list of the usernames who
 * have been invited to the game.
 * @param {array} invitee_usernames 
 */
const set_pending_invites = async invitee_usernames => {
    const invitees_ul = document.getElementById("invitees_ul")
    let list_html = ""

    invitee_usernames.map(invitee_username =>
        list_html += "<li>" +
            invitee_username +
            get_delete_invite_btn(invitee_username) +
            "</li>"
    )
    
    invitees_ul.innerHTML = list_html
}

const get_delete_invite_btn = invitee_username => {
    const uninvite_id = get_uninvite_id(invitee_username)
    let link_html = "<a href='#' class='uninvite_button' id='" + uninvite_id + "'>"
    link_html += "X"
    link_html += "</a>"

    return link_html
}

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

const get_uninvite_id = invitee_username => "uninvite_" + invitee_username

/**
 * When the owner presses the button to invite another player
 */
const invite_player = async () => {
    console.log("gonna invite")
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


document.addEventListener('DOMContentLoaded', () => {
    hide_msg_box()
    
    // get button elements so we can add event listeners
    const join_btn = document.getElementById('join_btn')
    const start_btn = document.getElementById('start_btn')
    const refresh_btn = document.getElementById('refresh_btn')
    const invite_button = document.getElementById('invite_submit')

    // Add event listeners
    join_btn && join_btn.addEventListener('click', (e) => join_game())
    start_btn && start_btn.addEventListener('click', (e) => start_game())
    refresh_btn && refresh_btn.addEventListener('click', (e) => refresh_data())
    invite_button && invite_button.addEventListener('click', (e) => invite_player())

    refresh_data()

    // Check every 3 seconds for new users or updated game_status
    setInterval(refresh_data, 3000);
})

window.join_game = join_game
window.start_game = start_game
window.refresh_data = refresh_data
window.invite_player = invite_player