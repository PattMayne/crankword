# CRANKWORD

Crankword is a turn-based multiplayer word-guessing game.

## TODO:
* Invalid Refresh Token should send user to login page.
* * Right now it just sends to error page.
* * We need a redirect flow which goes from needs-login to login back to current_game
* * Deal with refresh_token timing out DURING gameplay
* USER QUITS GAME:
* * Other players must also detect and reload (with popup message and timeout)
* * Turn must be set to a remaining player's turn
* * * when quit: if player's turn, switch turn before quit
* * * BETTER YET: cannot quit when it IS your turn