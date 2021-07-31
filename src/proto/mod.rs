use std::io::Cursor;
use prost::Message;

pub mod message {
    include!(concat!(env!("OUT_DIR"), "/chat.message.rs"));
}

pub fn deserialize_client_message(bytes: &[u8]) -> message::ClientMessage {
    message::ClientMessage::decode(&mut Cursor::new(bytes)).expect("deserialize_error")
}

pub fn serialize_server_message(message: message::ServerMessage) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf).expect("serialize_error");
    buf
}

