-- 0001_init.sql


CREATE TABLE IF NOT EXISTS games (
    id INT AUTO_INCREMENT NOT NULL UNIQUE,
    word VARCHAR(10) NOT NULL,
    game_status VARCHAR(20) NOT NULL DEFAULT "pre_game", -- options: pre_game, in_progress, finished, cancelled
    winner_id INT, -- nullable
    owner_id INT NOT NULL,
    turn_user_id INT NOT NULL DEFAULT 0,
    created_timestamp TIMESTAMP NOT NULL DEFAULT UTC_TIMESTAMP
);

CREATE INDEX idx_winner_id ON games(winner_id);


CREATE TABLE IF NOT EXISTS user_game_stats (
    user_id INT PRIMARY KEY,
    wins INT DEFAULT 0,
    losses INT DEFAULT 0
);


CREATE TABLE IF NOT EXISTS guesses (
    id INT AUTO_INCREMENT NOT NULL UNIQUE,
    game_id INT NOT NULL,
    word VARCHAR(10) NOT NULL,
    guess_number TINYINT NOT NULL,
    user_id INT NOT NULL
);


-- 
CREATE TABLE IF NOT EXISTS game_users (
    game_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES games(id)
    -- cannot do FOREIGN KEY on user_id b/c that's stored in auth_app DB
);
