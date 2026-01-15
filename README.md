# CRANKWORD

Crankword is a turn-based multiplayer word-guessing game.

## TODO:
* Invalid Refresh Token should send user to login page.
* * Right now it just sends to error page.
* * We need a redirect flow which goes from needs-login to login back to current_game
* Make one MySqlPool for the WHOLE application and store it in web::Data<MySqlPool>
 * * This is declared at the beginning of the routes/middleware chain
 * * THIS IS VERY IMPORTANT
* only make "enter" submit check_guess when a tile is focused (for accessibility)