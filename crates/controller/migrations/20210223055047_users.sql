-- Add migration script here
create table users
(
    id            serial not null
        constraint users_pk
            primary key,
    name          text   not null,
    email         text   not null,
    password_hash text   not null
);

alter table users
    owner to current_user;

create unique index users_email_uindex
    on users (email);

create unique index users_id_uindex
    on users (id);

create unique index users_name_uindex
    on users (name);


