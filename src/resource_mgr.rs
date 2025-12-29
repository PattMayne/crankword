/* 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  RESOURCE MANAGER  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * Gather translations into structs in this script
 * to keep that logic out of the routes script.
 * 
 * Some pages or templates require custom functions
 * to build their structs.
 * Most can simply use the get_translation function.
 * 
 * 
*/


use crate::{
    auth::UserReqData,
    resources::{ get_translation, raw_trans_or_missing, TRANSLATIONS },
    utils::SupportedLangs
};


/* 
 * 
 * 
 * 
 * 
 * ==================================
 * ==================================
 * =====                        =====
 * =====  TRANSLATIONS STRUCTS  =====
 * =====                        =====
 * ==================================
 * ==================================
 * 
 * 
 * Each askama template will have a struct
 * designed to hold all necessary text.
 * 
 * 
 * 
*/


pub struct DashTexts {
    pub title: String,
    pub new_game: String,
    pub current_games: String,
    pub stats: String,
    pub nav: NavTexts
}

impl DashTexts {
    pub fn new(user_req_data: &UserReqData) -> DashTexts {
        let lang: &SupportedLangs = &user_req_data.lang;
        let title: String = get_translation("home.title", lang, None);
        let stats: String = get_translation("dash.stats", lang, None);
        let new_game: String = get_translation("dash.new_game", lang, None);
        let current_games: String = get_translation(
            "dash.current_games",
            lang,
            None
        );
        let nav: NavTexts = NavTexts::new(lang);

        DashTexts { title, new_game, stats, current_games, nav }
    }
}


/*

current_games
stats

*/

/**
 * route: get "/"
 */
pub struct HomeTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl HomeTexts {
    pub fn new(user_req_data: &UserReqData) -> HomeTexts {
        let lang: &SupportedLangs = &user_req_data.lang;
        let title: String = get_translation("home.title", lang, None);
        let message: String = get_translation(
            "home.greeting",
            lang,
            Some(&[&user_req_data.get_role()]));
        let nav: NavTexts = NavTexts::new(lang);

        HomeTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/error"
 */
pub struct ErrorTexts {
    pub nav: NavTexts
}

impl ErrorTexts {
    pub fn new(user_req_data: &UserReqData) -> ErrorTexts {
        let nav = NavTexts::new(&user_req_data.lang);

        ErrorTexts {
            nav
        }
    }
}


/* 
 * 
 * 
 * 
 * 
 * =====================
 * =====================
 * =====           =====
 * =====  TOP NAV  =====
 * =====           =====
 * =====================
 * =====================
 * 
 * 
 * 
 * 
 * The top-nav bar is loaded on every page, so here is a struct to gather
 * all of its button translations together.
 * They can be static references because they will never build by replacing
 * placeholders. Simple strings.
 */
pub struct NavTexts {
    pub home: &'static str,
    pub admin: &'static str,
    pub dashboard: &'static str,
    pub login: &'static str,
    pub register: &'static str,
    pub logout: &'static str,
}


impl NavTexts {

    /**
     * Just pass in a language to this constructor and get the right language version
     * of all the strings for the top-nav buttons.
     */
    pub fn new(lang: &SupportedLangs) -> NavTexts {
        let lang_suffix: &str = lang.suffix();

        let home_key: String = format!("{}.{}", "nav.home", lang_suffix);
        let admin_key: String = format!("{}.{}", "nav.admin", lang_suffix);
        let dash_key: String = format!("{}.{}", "nav.dashboard", lang_suffix);
        let login_key: String = format!("{}.{}", "nav.login", lang_suffix);
        let register_key: String = format!("{}.{}", "nav.register", lang_suffix);
        let logout_key: String = format!("{}.{}", "nav.logout", lang_suffix);

        let home: &'static str = raw_trans_or_missing(home_key.as_str(), lang);
        let admin: &'static str = raw_trans_or_missing(admin_key.as_str(), lang);
        let dashboard: &'static str = raw_trans_or_missing(dash_key.as_str(), lang);
        let login: &'static str = raw_trans_or_missing(login_key.as_str(), lang);
        let register: &'static str = raw_trans_or_missing(register_key.as_str(), lang);
        let logout: &'static str = raw_trans_or_missing(logout_key.as_str(), lang);

        NavTexts {
            home,
            admin,
            dashboard,
            login,
            register,
            logout,
        }
    }
}


/* 
 * 
 * 
 * 
 * 
 * =========================
 * =========================
 * =====               =====
 * =====  ERROR CODES  =====
 * =====               =====
 * =========================
 * =========================
 * 
 * 
 * 
 * Custom logic to get Error page text.
 * The "custom" part is getting default data for
 * unknown or invalid error codes.
 * 
*/


// Text for Error page
pub struct ErrorData {
    pub code: String,
    pub title: &'static str,
    pub message: &'static str,
}

impl ErrorData {
    pub fn new(code: String, lang: &SupportedLangs) -> Self {
        let lang_suffix: &str = lang.suffix();
        let title_key: String = format!("{}.{}.{}.{}", "err", code, "title", lang_suffix);
        let body_key: String = format!("{}.{}.{}.{}", "err", code, "body", lang_suffix);

        // Get the option first so we can check if it's a known error code
        let title_option: Option<&&str> = TRANSLATIONS.get(title_key.as_str());
        let body_option: Option<&&str> = TRANSLATIONS.get(body_key.as_str());

        // Just hardcode the missing errors here
        if title_option.is_none() || body_option.is_none() {
            match lang {
                SupportedLangs::English => {
                    return ErrorData {
                        code: code,
                        title: "Unknown Error",
                        message: "An unknown error has occurred.",
                    };
                },
                SupportedLangs::French => {
                    return ErrorData {
                        code: code,
                        title: "Erreur inconnue",
                        message: "Une erreur inconnue s'est produite.",
                    };
                }
            }
        }

        // The error code is known, text is retrieved. Create and return struct.
        ErrorData {
            code: code,
            title: title_option.unwrap(),
            message: body_option.unwrap(),
        }
    }
}

fn missing_error(lang: &SupportedLangs) -> &'static str {
    match lang {
        SupportedLangs::English => "Error",
        SupportedLangs::French => "Erreur"
    }
}

/**
 * Uses the title of the Error Page error data for simple error messages.
 */
pub fn error_by_code(code: String, lang: &SupportedLangs) -> &'static str {
    let key: String = format!("{}.{}.{}.{}", "err", code, "title", lang.suffix());

    match TRANSLATIONS.get(&key) {
        Some(translation) => translation,
        None => missing_error(lang)
    }
}