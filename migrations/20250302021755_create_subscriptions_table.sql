-- Add migration script here
CREATE TABLE subscriptions (
    seqno bigserial not null,
    primary key (seqno),
    email varchar(255) not null unique,
    name varchar(30) not null,
    subscribed_at timestamptz not null default current_timestamp
);