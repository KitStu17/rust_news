-- Add migration script here
CREATE TABLE subscriptions (
    id uuid not null,
    primary key (id),
    email varchar(255) not null unique,
    name varchar(30) not null,
    subscribed_at timestamptz not null default current_timestamp
);

comment on column subscriptions.id is '식별자';
comment on column subscriptions.email is '구독 이메일';
comment on column subscriptions.name is '구독자 이름';
comment on column subscriptions.subscribed_at is '구독일';