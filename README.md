# CRANKWORD

Crankword is a turn-based multiplayer word-guessing game.

## TODO:
* Invalid Refresh Token should send user to login page.
* * Right now it just sends to error page.
* * We need a redirect flow which goes from needs-login to login back to current_game
* Find a way to share games or find games.
* * Messages in inbox?
* * Invitations?
* * * Start typing and hit "invite"
* * * Invite code/URL?
* Invite codes (actually the masked ID)
* * code and route (URL) to read code
* * In-game invitation
* * Send email invitations (only will work when online)
* * * will probably have to go through crankade auth app
* Timeout miss a turn should trigger a popup
* * This requires storing player's username somewhere at game start