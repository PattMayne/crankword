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
    "err.503.body.fr" => "Le serveur n'est pas prêt à gérer la demande. Veuillez revenir plus tard.",

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