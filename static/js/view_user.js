$(document).foundation()
import * as io from './io.js'
import * as utils from './utils.js'
import * as globals from './globals.js'


let msgs = []

const block_user = () => {
    // get username
    // call the API
    // show result

    console.log("BLOCKING USER")
}

// SHOW/HIDE ERROR BOX

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";

    for (let msg of msgs) {
        const msg_p = "<p>" + msg + "</p>"
        msg_box.innerHTML += msg_p
    }

    show_element(msg_box)
}

const show_element = element => element.style.display = ""
const hide_element = element => element.style.display = "none"


// Add event listeners
document.addEventListener('DOMContentLoaded', () => {
    hide_element(document.getElementById("msg_box"))

})

document.getElementById('block_user_btn').addEventListener(
    'click', (e) => block_user())


window.block_user = block_user