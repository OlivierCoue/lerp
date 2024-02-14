
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS world_instances;


CREATE TABLE users (
    uuid UUID NOT NULL,
    username VARCHAR(200) NOT NULL,
    current_world_instance_uuid UUID NULL,
    auth_token UUID NULL,

    CONSTRAINT pk_users PRIMARY KEY (uuid)
);

CREATE UNIQUE INDEX uidx_users_username ON users(UPPER(username));
CREATE UNIQUE INDEX uidx_users_auth_token ON users(auth_token);

CREATE TABLE world_instances (
    uuid UUID NOT NULL,
    created_by UUID NOT NULL,

    CONSTRAINT world_instances_pkey PRIMARY KEY (uuid)
);

ALTER TABLE world_instances ADD CONSTRAINT fk_created_by_users FOREIGN KEY (created_by) REFERENCES users(uuid);
CREATE INDEX idx_created_by_users ON world_instances(created_by);

ALTER TABLE users ADD CONSTRAINT fk_current_world_instance_uuid_world_instances FOREIGN KEY (current_world_instance_uuid) REFERENCES world_instances(uuid) ON DELETE SET NULL ON UPDATE CASCADE;
CREATE INDEX idx_users_current_world_instance_uuid ON users(current_world_instance_uuid);