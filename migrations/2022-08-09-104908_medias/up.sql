-- Your SQL goes here
create table types
(
    id         serial       not null primary key,
    name       varchar(100) not null,
    created_at timestamp without time zone default now(),
    updated_at timestamp without time zone default now(),
    deleted_at timestamp without time zone
);

create table medias
(
    id         bigserial    not null primary key,
    fk_user_id bigint       not null,
    fk_type_id int          not null,
    directory  varchar(200) not null,
    media      text         not null,
    other_data jsonb,
    created_at timestamp without time zone default now(),
    updated_at timestamp without time zone default now(),
    deleted_at timestamp without time zone,
    foreign key (fk_user_id) references users (id) on delete restrict on update cascade,
    foreign key (fk_type_id) references types (id) on delete restrict on update cascade
);
