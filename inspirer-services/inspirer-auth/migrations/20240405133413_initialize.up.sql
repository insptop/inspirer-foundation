-- domains
create table
    if not exists domains (
        id int unsigned not null auto_increment primary key,
        uuid binary(16) not null,
        name varchar(100) not null,
        display_name varchar(100) not null,
        profile json,
        created_at timestamp not null,
        updated_at timestamp not null
    );

create unique index unique_domain_uuid on domains (uuid);

create unique index unique_domain_name on domains (name);

-- users
create table
    if not exists users (
        id bigint unsigned not null auto_increment primary key,
        uuid binary(16) not null,
        domain_uuid binary(16) not null,
        email varchar(120) default null,
        email_verified boolean not null default false,
        username varchar(120) default null,
        phone_number varchar(60) default null,
        phone_number_verified boolean not null default false,
        password varchar(120) not null default '',
        profile json not null,
        created_at timestamp not null,
        updated_at timestamp not null
    );

create index index_domain on users (domain_uuid);

create unique index unique_user_uuid on users (uuid);

create unique index unique_email on users (email);

create unique index unique_phone on users (phone_number);

create unique index unique_username on users (username);

-- apps
create table
    if not exists apps (
        id int unsigned not null auto_increment primary key,
        uuid binary(16) not null,
        secret binary(16) not null comment 'App secret',
        domain_uuid binary(16) not null,
        name varchar(100) not null,
        display_name varchar(100) not null,
        profile json not null,
        setting json not null,
        created_at timestamp not null,
        updated_at timestamp not null
    );

create unique index unique_app_uuid on apps (uuid);

create index index_domain on apps (domain_uuid);

create unique index unique_app_name on apps (name);