$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []
let invites_list = null

const create_new_game = async () => {
    console.log("CREATING NEW GAME")
    msgs = []
    msgs.push("NEW GAME CREATING")

    const invite_only = document.getElementById("invite_only_check").checked

    const game_data = await io.new_game(invite_only)
    const hashed_game_id = game_data.hashed_game_id

    if (!hashed_game_id || hashed_game_id < 1) {
        msgs = []
        msgs.push(game_data.error)
        show_msg_box()
        return
    }

    // Redirect user to game
    const game_uri = "/game/" + hashed_game_id
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


const show_element = element => element.style.display = ""
const hide_element = element => element.style.display = "none"

const toggle_rules = () => {
    const rules_box = document.getElementById("rules_div")
    const rules_btn = document.getElementById("toggle_rules")
    if (rules_box.style.display != "none") {
        hide_element(rules_box)
        rules_btn.innerHTML = "[+]"
    } else {
        show_element(rules_box)
        rules_btn.innerHTML = "[-]"
        rules_box.scrollIntoView({ behavior: 'smooth' })
    }
}


/**
 * Check for new invitations somebody might have sent.
 */
const refresh_data = async () => {
    if (invites_list == null || !! invites_list) {
        invites_list = document.getElementById("invitations")
    }

    // from io get list of game ids
    const invited_games_obj = await io.refresh_dashboard()
    const username = document.getElementById("username").value
    
    if (!invited_games_obj.invited_games || invited_games_obj.invited_games.length < 1) {
        invites_list.innerHTML = "[NONE]"
        return
    }

    const invited_games = invited_games_obj.invited_games
    const game_ids_html = invited_games.reduce((html, game) => {
        return html + "<div class='callout invite_callout'>" +
            "<a class='button small invite_btn' href='/game/" +
            game.hashid + "'>" + game.hashid + "</a>" +
            "<h6><a href='/user/" + game.owner_name +"'>" + game.owner_name + "</a></h6>" +
            "<a class='decline_invite' id='" + get_decline_id(game.hashid) +
            "'>[decline]</a>" + "</div>"
    }, "")
    invites_list.innerHTML = game_ids_html
    set_decline_event_listeners(invited_games, username)
}

const set_decline_event_listeners = async (invited_games, username) => {
    invited_games.map(game => {
        document.getElementById(get_decline_id(game.hashid))
            .addEventListener('click', (e) => {
                io.uninvite_player(game.hashid, username).then(result => {
                    msgs.push(result.message)
                    show_msg_box()
                    msgs = []
                    refresh_data()
                })
            })
        })
}

const get_decline_id = hashid => "decline_" + hashid

// Add event listeners
document.addEventListener('DOMContentLoaded', () => {
    hide_element(document.getElementById("rules_div"))
    hide_msg_box()
    invites_list = document.getElementById("invitations")
    refresh_data()

    // Check every 4 seconds for new users or updated game_status
    setInterval(refresh_data, 4001)
})

document.getElementById('new_game_button').addEventListener(
    'click', (e) => create_new_game())

document.getElementById('toggle_rules').addEventListener(
    'click', (e) => toggle_rules())

window.create_new_game = create_new_game