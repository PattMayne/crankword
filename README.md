# CRANKWORD

Crankword is a turn-based multiplayer word-guessing game.

## TODO:
* Invalid Refresh Token should send user to login page.
* * Right now it just sends to error page.
* * We need a redirect flow which goes from needs-login to login back to current_game
* * Deal with refresh_token timing out DURING gameplay
* Main page must actually look good
* * Pictures of the gameplay style
* POST calls should all be able to return the SAME error struct for simplicity
