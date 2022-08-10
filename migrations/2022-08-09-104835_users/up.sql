-- Your SQL goes here
create table users
(
    id         bigserial not null primary key,
    created_at timestamp without time zone default now(),
    updated_at timestamp without time zone default now(),
    deleted_at timestamp without time zone
);
