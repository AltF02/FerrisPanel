-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table servers
(
    id          uuid default uuid_generate_v4() not null
        constraint servers_pk
            primary key,
    name        text                            not null,
    description text                            not null
);

alter table servers
    owner to postgres;

create unique index servers_id_uindex
    on servers (id);

