
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS game_worlds;
DROP TABLE IF EXISTS game_areas;
DROP TABLE IF EXISTS game_servers;

-- USERS --

CREATE TABLE users (
    uuid UUID NOT NULL,
    username VARCHAR(200) NOT NULL,
    current_game_world_uuid UUID NULL,
    current_game_area_uuid UUID NULL,
    auth_token UUID NULL,
    game_server_handshake_challenge UUID NULL,

    CONSTRAINT pk_users PRIMARY KEY (uuid)
);

CREATE UNIQUE INDEX uidx_users_username ON users(UPPER(username));
CREATE UNIQUE INDEX uidx_users_auth_token ON users(auth_token);
CREATE UNIQUE INDEX uidx_users_game_server_handshake_challenge ON users(game_server_handshake_challenge);

-- GAME WORLDS --

CREATE TABLE game_worlds (
    uuid UUID NOT NULL,
    created_by UUID NOT NULL,

    CONSTRAINT game_worlds_pkey PRIMARY KEY (uuid)
);

ALTER TABLE game_worlds ADD CONSTRAINT fk_game_worlds_created_by_users FOREIGN KEY (created_by) REFERENCES users(uuid);
CREATE INDEX idx_game_worlds_created_by_users ON game_worlds(created_by);

ALTER TABLE users ADD CONSTRAINT fk_current_game_world_uuid_game_worlds FOREIGN KEY (current_game_world_uuid) REFERENCES game_worlds(uuid) ON DELETE SET NULL ON UPDATE CASCADE;
CREATE INDEX idx_users_current_game_world_uuid ON users(current_game_world_uuid);

-- GAME AREAS --

CREATE TABLE game_areas (
    uuid UUID NOT NULL,
    created_by UUID NOT NULL,

    CONSTRAINT game_areas_pkey PRIMARY KEY (uuid)
);

ALTER TABLE game_areas ADD CONSTRAINT fk_game_areas_created_by_users FOREIGN KEY (created_by) REFERENCES users(uuid);
CREATE INDEX idx_game_areas_created_by_users ON game_areas(created_by);

ALTER TABLE users ADD CONSTRAINT fk_current_game_area_uuid_game_areas FOREIGN KEY (current_game_area_uuid) REFERENCES game_areas(uuid) ON DELETE SET NULL ON UPDATE CASCADE;
CREATE INDEX idx_users_current_game_area_uuid ON users(current_game_area_uuid);

-- GAME SERVER --

CREATE TABLE game_servers (
    uuid UUID NOT NULL,
    udp_port INTEGER NOT NULL,
    aes_key VARCHAR(64) NOT NULL,
    aes_nonce VARCHAR(12) NOT NULL,

    CONSTRAINT game_servers_pkey PRIMARY KEY (uuid)
)