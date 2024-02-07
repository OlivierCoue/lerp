
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS world_instances;

CREATE TABLE users (
    uuid UUID NOT NULL,
    username VARCHAR(200) NOT NULL,
    current_world_instance_uuid UUID NULL,

    CONSTRAINT pk_users PRIMARY KEY (uuid)
);

CREATE UNIQUE INDEX uidx_users_username ON users(UPPER(username));

CREATE TABLE world_instances (
    uuid UUID NOT NULL,

    CONSTRAINT world_instances_pkey PRIMARY KEY (uuid)
);

ALTER TABLE users ADD CONSTRAINT fk_current_world_instance_uuid_world_instances FOREIGN KEY (current_world_instance_uuid) REFERENCES world_instances(uuid) ON DELETE CASCADE ON UPDATE CASCADE;
CREATE INDEX idx_users_current_world_instance_uuid ON users(current_world_instance_uuid);