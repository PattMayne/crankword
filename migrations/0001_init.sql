-- 0001_init.sql


CREATE TABLE IF NOT EXISTS games (
    id INT AUTO_INCREMENT NOT NULL UNIQUE,
    word VARCHAR(10) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT "ongoing", -- options: ongoing, finished, cancelled
    winner_id INT, -- nullable
    created_timestamp TIMESTAMP NOT NULL DEFAULT UTC_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS guesses (
    id INT AUTO_INCREMENT NOT NULL UNIQUE,
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
