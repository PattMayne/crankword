use phf::phf_map;

use crate::utils::SupportedLangs;

/* 
 * 
 * 
 * 
 * 
 * =======================
 * =======================
 * =====             =====
 * =====  RESOURCES  =====
 * =====             =====
 * =======================
 * =======================
 * 
 * 
 * Text strings and functions to retrieve them.
 * Text is stored in static string references in a phf_map constant.
 * They can be retrieved by calling their keys.
 * 
 * We will have French and English versions of everything.
 * 
 * 
 * 
*/


/**
 * Text strings to use on website.
 * Placeholders must be NUMBERED starting with ZERO:
 * ie:  Hello, {0}! I hope you're having a good {1}!
 * --> where {0} and {1} can later be replaced with username and Morning/Afternoon
 * 
 * If using HTML tags, must add "safe" in askama template: * 
 *                  {{ text | safe }}
 * 
 */
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {

    // HOME PAGE
    "home.title.fr" => "Crankword",
    "home.title.en" => "Crankword",
    "home.greeting.en" => "Hello, {0}!",
    "home.greeting.fr" => "Bonjour, {0}!",
    "home.message.1.en" => "Welcome to CRANKWORD!",
    "home.message.1.fr" => "Bienvenue chez CRANKWORD !",
    "home.message.2.en" => "Crankword is a multi-player, turn-based, word-guessing game.
        Start a new game and invite your friends, join an open game, or practice against yourself!",
    "home.message.2.fr" => "Crankword est un jeu multijoueur où l'on devine des mots.
        Commencez une nouvelle partie et invitez vos amis, ou rejoignez une partie ouverte.
        Ou joue contre toi-même !",

    // DASH TEXTS
    "dash.title.en" => "DASHBOARD",
    "dash.title.fr" => "TABLEAU DE BORD",
    "dash.new_game.en" => "CREATE NEW GAME",
    "dash.new_game.fr" => "NOUVEAU JEU",
    "dash.current_games.en" => "CURRENT GAMES",
    "dash.current_games.fr" => "JEUX ACTUELS",
    "dash.stats.en" => "STATS",
    "dash.stats.fr" => "STATISTIQUES",

    // NAV BUTTONS
    "nav.home.en" => "HOME",
    "nav.home.fr" => "ACCUEIL",
    "nav.admin.en" => "ADMIN",
    "nav.admin.fr" => "ADMIN",
    "nav.login.en" => "LOGIN",
    "nav.login.fr" => "CONNEXION",
    "nav.register.en" => "REGISTER",
    "nav.register.fr" => "INSCRIPTION",
    "nav.logout.en" => "LOGOUT",
    "nav.logout.fr" => "DÉCONNEXION",
    "nav.dashboard.en" => "DASHBOARD",
    "nav.dashboard.fr" => "TABLEAU DE BORD",

    // PRE-GAME PAGE TEXTS
    "pregame.players.label.en" => "PLAYERS",
    "pregame.players.label.fr" => "JOUEURS",

    // IN-GAME PAGE TEXTS
    "ingame.cancel.btn.cancel.en" => "CANCEL GAME",
    "ingame.cancel.btn.cancel.fr" => "CANCEL GAME",
    "ingame.cancel.btn.quit.en" => "QUIT GAME",
    "ingame.cancel.btn.quit.fr" => "QUIT GAME",


    "ingame.cancel.confirm.cancel.en" => "Are you sure you want to cancel?",
    "ingame.cancel.confirm.cancel.fr" => "Are you sure you want to cancel?",
    "ingame.cancel.confirm.quit.en" => "Are you sure you want to quit?",
    "ingame.cancel.confirm.quit.fr" => "Are you sure you want to quit?",

    // POST-GAME PAGE TEXTS
    "postgame.winner.message.en" => "Game over! {0} is the winner!",
    "postgame.winner.message.fr" => "Fin du jeu !{0} est le gagnant !",
    "postgame.nowinner.message.en" => "Game over! There was no winner!",
    "postgame.nowinner.message.fr" => "Fin du jeu ! Il n'y avait pas de gagnant !",
    "postgame.cancelled.message.en" => "Game was cancelled!",
    "postgame.cancelled.message.fr" => "Le jeu a été annulé !",
    

    // ERROR CODES AND TITLES FOR ERROR PAGE
    "err.400.title.en" => "Bad Request",
    "err.400.title.fr" => "Mauvaise demande",
    "err.400.body.en" => "The request was malformed or otherwise bad.",
    "err.400.body.fr" => "La demande était mal formulée ou autrement mauvaise.",

    "err.401.title.en" => "Unauthorized",
    "err.401.title.fr" => "Non autorisé",
    "err.401.body.en" => "User is not authenticated.",
    "err.401.body.fr" => "L'utilisateur n'est pas authentifié.",

    "err.403.title.en" => "Forbidden",
    "err.403.title.fr" => "Interdit",
    "err.403.body.en" => "You do not have permission to view this page.",
    "err.403.body.fr" => "Vous n'avez pas la permission de consulter cette page.",

    "err.404.title.en" => "Not Found",
    "err.404.title.fr" => "Non trouvé",
    "err.404.body.en" => "The page you are looking for was not found.",
    "err.404.body.fr" => "La page que vous cherchez n'a pas été trouvée.",

    "err.408.title.en" => "Request Timeout",
    "err.408.title.fr" => "Délai de demande",
    "err.408.body.en" => "Server is shutting down connection.",
    "err.408.body.fr" => "Le serveur coupe la connexion.",

    "err.409.title.en" => "Conflict",
    "err.409.title.fr" => "Conflit",
    "err.409.body.en" => "Unacceptable duplicate input.",
    "err.409.body.fr" => "Entrée doublée inacceptable.",

    "err.422.title.en" => "Unprocessable Content",
    "err.422.title.fr" => "Contenu non traitable",
    "err.422.body.en" => "Request was well formed but content contains semantic errors.",
    "err.422.body.fr" => "La demande était bien formulée, mais le contenu contient des erreurs sémantiques.",

    "err.429.title.en" => "Too Many Requests",
    "err.429.title.fr" => "Trop de demandes",
    "err.429.body.en" => "User has sent too many requests.",
    "err.429.body.fr" => "L'utilisateur a envoyé trop de demandes.",

    "err.500.title.en" => "Internal Server Error",
    "err.500.title.fr" => "Erreur interne du serveur",
    "err.500.body.en" => "An unexpected error occurred.",
    "err.500.body.fr" => "Une erreur inattendue s'est produite.",

    "err.502.title.en" => "Bad Gateway",
    "err.502.title.fr" => "Mauvaise passerelle",
    "err.502.body.en" => "Gateway server received an invalid response.",
    "err.502.body.fr" => "Le serveur passerelle a reçu une réponse invalide.",

    "err.503.title.en" => "Service Unavailable",
    "err.503.title.fr" => "Service indisponible",
    "err.503.body.en" => "Server is not ready to handle the request. Please check back later.",
    "err.503.body.fr" => "Le serveur n'est pas prêt à gérer la demande. 
        Veuillez revenir plus tard.",

    "err.504.title.en" => "Gateway Timeout",
    "err.504.title.fr" => "Délai d'attente de la passerelle",
    "err.504.body.en" => "Server did not respond in time.",
    "err.504.body.fr" => "Le serveur n'a pas répondu à temps.",

    // AD-HOC ERRORS FOR JSON
    "err.empty_creds.en" => "Invalid Credentials: Empty Field.",
    "err.empty_creds.fr" => "Identifiants invalides : champ vide.",
    "err.invalid_creds.en" => "Invalid Credentials.",
    "err.invalid_creds.fr" => "Identifiants invalides.",
    "err.user_not_found.en" => "User not found.",
    "err.user_not_found.fr" => "Utilisateur non trouvé.",

    // LINKS & URLS
    "links.login.en" => "http://auth.localhost.test:3000/auth/login",
    "links.login.fr" => "http://auth.localhost.test:3000/auth/login",
    "links.register.en" => "http://auth.localhost.test:3000/auth/register",
    "links.register.fr" => "http://auth.localhost.test:3000/auth/register",

    // EXPLAINER / RULES
    "rules.title.en" => "HOW IT WORKS",
    "rules.title.fr" => "COMMENT ÇA FONCTIONNE",


    "rules.body.en" => "<h3>CREATING A GAME</h3>
        <p>You can start a game in the DASHBOARD. 
        When you start a game you are the OWNER of the game. 
        The OWNER can invite other players, kick players out who joined, cancel a game, 
        and start a game. The game can be INVITE-ONLY, or OPEN. If it's INVITE-ONLY, the owner 
        must invite people by entering their usernames in the field on the PRE-GAME dashboard. 
        If it's an OPEN game, it will appear on the OPEN GAMES page, and anybody can join.</p>
        <p>A game has three phases: PRE-GAME, IN-PROGRESS, and COMPLETED. 
        During the PRE-GAME phase people can join the game and wait for the OWNER to start the game.</p>
        <h3>JOINING A GAME</h3>
        <p>You can join a game by either accepting an invitation which appears on your 
        dashboard, or you can go to OPEN GAMES in the top nav and joining a game there.</p>
        <p>The OPEN GAMES are sorted by age. If you join a game but the OWNER takes too long 
        to start the game, you can leave the game from the IN-PROGRESS page. If you have too many 
        invitations, you can decline an invitation in the IN-PROGRESS page by pressing the (x) 
        next to your username in the PENDING INVITATIONS list.</p>
        <h3>PLAYING THE GAME</h3>
        <p>For each game there is a secret five-letter word. Each player will try to guess 
        that word. It's turn-based, so you have to wait until everybody else guesses before 
        you can guess again. Everybody gets five guesses.</p>
        <p>There is a timer so nobody takes too long. If the timer runs out, the game skips 
        that turn and it goes to the next player. However, this only works if the game's OWNER 
        is online. If you find that people are taking too long, and the OWNER is absent, you 
        can QUIT a game where nobody has made a guess in five minutes. But if the game is active 
        then you cannot QUIT an IN-PROGRESS game.</p>",

    "rules.body.fr" => "<h2>INSTRUCTIONS FRANÇAISES EN COURS DE CONSTRUCTION !</h2>
        <h3>CREATING A GAME</h3>
        <p>You can start a game in the DASHBOARD. 
        When you start a game you are the OWNER of the game. 
        The OWNER can invite other players, kick players out who joined, cancel a game, 
        and start a game. The game can be INVITE-ONLY, or OPEN. If it's INVITE-ONLY, the owner 
        must invite people by entering their usernames in the field on the PRE-GAME dashboard. 
        If it's an OPEN game, it will appear on the OPEN GAMES page, and anybody can join.</p>
        <p>A game has three phases: PRE-GAME, IN-PROGRESS, and COMPLETED. 
        During the PRE-GAME phase people can join the game and wait for the OWNER to start the game.</p>
        <h3>JOINING A GAME</h3>
        <p>You can join a game by either accepting an invitation which appears on your 
        dashboard, or you can go to OPEN GAMES in the top nav and joining a game there.</p>
        <p>The OPEN GAMES are sorted by age. If you join a game but the OWNER takes too long 
        to start the game, you can leave the game from the IN-PROGRESS page. If you have too many 
        invitations, you can decline an invitation in the IN-PROGRESS page by pressing the (x) 
        next to your username in the PENDING INVITATIONS list.</p>
        <h3>PLAYING THE GAME</h3>
        <p>For each game there is a secret five-letter word. Each player will try to guess 
        that word. It's turn-based, so you have to wait until everybody else guesses before 
        you can guess again. Everybody gets five guesses.</p>
        <p>There is a timer so nobody takes too long. If the timer runs out, the game skips 
        that turn and it goes to the next player. However, this only works if the game's OWNER 
        is online. If you find that people are taking too long, and the OWNER is absent, you 
        can QUIT a game where nobody has made a guess in five minutes. But if the game is active 
        then you cannot QUIT an IN-PROGRESS game.</p>",
};


/**
 * For missing translations, or mis-typed keys.
 */
fn missing_trans(lang: &SupportedLangs) -> &'static str {
    match lang {
        SupportedLangs::English => "[ translation missing ]",
        SupportedLangs::French => "[ traduction manquante ]"
    }
}


/**
 * Take the keyword for the translation to call,
 * a language enum so we know the language suffix to add,
 * and an optional set of &str slices for placeholder phrases
 */
pub fn get_translation(
    keyword: &str,
    lang: &SupportedLangs,
    params_option: Option<&[&str]>
) -> String {
    let full_key: String = format!("{}.{}", keyword, lang.suffix());
    
    match TRANSLATIONS.get(full_key.as_str()) {
        Some(translation) => {
            let translation: String = translation.to_string();

            // replace placeholders with text from args
            match params_option {
                None => translation,
                Some(args) => {
                    let mut translation: String = translation;
                    for (i, arg) in args.iter().enumerate() {
                        let placeholder: String = format!("{{{}}}", i);
                        translation = translation.replace(&placeholder, arg);
                    }
                    translation
                }
            }
        },
        None => missing_trans(lang).to_string()
    }
}


/**
 * A quick and dirty retrieval of translations
 * which do NOT have placeholders.
 * Primarily for the nav translations.
 */
pub fn raw_trans_or_missing(
    key: &str,
    lang: &SupportedLangs
) -> &'static str {
    match TRANSLATIONS.get(key) {
        Some(translation) => translation,
        None => missing_trans(lang)
    }
}
