create table "User"
(
    id   int          not null primary key,
    name varchar(100) not null
);

create table Private_Message
(
    id           int  not null primary key,
    sender_id    int references "User",
    recipient_id int references "User",
    msg          text not null
);

create table Room
(
    id   int          not null primary key,
    name varchar(100) not null
);

create table Participant
(
    room_id        int references Room,
    participant_id int references "User",
    primary key (room_id, participant_id)
);

create table Room_Message
(
    id        int  not null primary key,
    room_id   int references Room,
    sender_id int references "User",
    msg       text not null
);
