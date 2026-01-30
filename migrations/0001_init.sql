-- 0001_init.sql


CREATE TABLE IF NOT EXISTS games (
    id INT AUTO_INCREMENT NOT NULL UNIQUE,
    word VARCHAR(10) NOT NULL,
    game_status VARCHAR(20) NOT NULL DEFAULT "pre_game", -- options: pre_game, in_progress, finished, cancelled
    winner_id INT, -- nullable
    owner_id INT NOT NULL,
    turn_user_id INT, -- nullable
    turn_timeout TIMESTAMP NOT NULL DEFAULT UTC_TIMESTAMP,
    open_game BOOL NOT NULL DEFAULT FALSE,
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

CREATE INDEX idx_game_id ON guesses(game_id);

-- 
CREATE TABLE IF NOT EXISTS game_users (
    game_id INT NOT NULL,
    user_id INT NOT NULL,
    username VARCHAR(255) NOT NULL,
    turn_order INT DEFAULT 0,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES games(id)
    -- cannot do FOREIGN KEY on user_id b/c that's stored in auth_app DB
);


CREATE TABLE IF NOT EXISTS invites (
    game_id INT NOT NULL,
    username VARCHAR(255) NOT NULL,
    PRIMARY KEY (game_id, username),
    FOREIGN KEY (game_id) REFERENCES games(id)
    -- cannot do FOREIGN KEY on user_id b/c that's stored in auth_app DB
);