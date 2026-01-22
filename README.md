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
* Show "current games" in dashboard
* Find a way to share games or find games.
* * Messages in inbox?
* * Invitations?
* * * Start typing and hit "invite"
* * * Invite code/URL?
* Mask ID?
* * three randomized numbers followed by ( ID * 374 ) plus three randomized numbers
* * * Then to read the URL just remove first 3, last 3, and divide by 374
* Invite codes (actually the masked ID)
* * code and route (URL) to read code
* * In-game invitation
* * Send email invitations (only will work when online)
* * * will probably have to go through crankade auth app