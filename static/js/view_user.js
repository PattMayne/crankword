$(document).foundation()
import * as utils from './utils.js'


let msgs = []

const block_user = async () => {
    // get username
    // call the API
    // show result

    const username_to_block = document.getElementById("username").value
    console.log("BLOCKING " + username_to_block)


    const route = "/block_user"

    const input = {
        "username": String(username_to_block)
    }

    const response_obj = {
        success: false,
        message: null
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to block user, or error on server.")
        }
        return response.json()
    }).then(data => {
        response_obj.message = data.message
        response_obj.success = data.success      
    }).catch(error => {
        console.log('Error: ', error)
    })

    msgs.push(response_obj.message)
    show_msg_box()

    setTimeout(() => {
        location.reload()
    }, 2500)
}


const unblock_user = async () => {
    // get username
    // call the API
    // show result

    const username_to_unblock = document.getElementById("username").value
    console.log("UNBLOCKING " + username_to_unblock)


    const route = "/unblock_user"

    const input = {
        "username": String(username_to_unblock)
    }

    const response_obj = {
        success: false,
        message: null
    }

    await utils.fetch_json_post(route, input)
    .then(response => {
        if(!response.ok) {
            response.json().then(data => {
                console.log("NOT OK")
                let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                msg += (!!data.error) ? data.error : " Error occurred"
                response_obj.error = msg
            })

            throw new Error("Unable to unblock user, or error on server.")
        }
        return response.json()
    }).then(data => {
        response_obj.message = data.message
        response_obj.success = data.success      
    }).catch(error => {
        console.log('Error: ', error)
    })

    msgs.push(response_obj.message)
    show_msg_box()

    setTimeout(() => {
        location.reload()
    }, 2500)
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
    msgs = []
}

const show_element = element => element.style.display = ""
const hide_element = element => element.style.display = "none"


// Add event listeners
document.addEventListener('DOMContentLoaded', () => {
    hide_element(document.getElementById("msg_box"))

    const block_button = document.getElementById('block_user_btn')
    const unblock_button = document.getElementById('unblock_user_btn')
    !!block_button && block_button.addEventListener('click', (e) => block_user())
    !!unblock_button && unblock_button.addEventListener('click', (e) => unblock_user())

})


window.block_user = block_user