create table public.user
(
    id   serial primary key,
    name varchar(100) not null,
    password varchar(100) not null,
    avatar varchar(200),
    last_session timestamp not null
);

create table private_message
(
    id           serial primary key,
    sender_id    int not null references public.user,
    recipient_id int not null references public.user,
    msg          text not null,
    sending_time timestamp not null,
    replayed_message_id serial references private_message
);

create table room
(
    id   serial primary key,
    name varchar(100) not null,
    avatar varchar(200),
    creator_id serial not null references public.user
);

create table participant
(
    room_id        int not null references Room,
    user_id int not null references public.user,
    primary key (room_id, user_id)
);

create table room_message
(
    id        serial  not null primary key,
    room_id   serial not null references room,
    sender_id serial references public.user,
    msg       text not null,
    replayed_message_id serial references room_message
);

create table contact
(
    user_id    int references public.user,
    contact_id int references public.user,
    primary key (user_id, contact_id)
);