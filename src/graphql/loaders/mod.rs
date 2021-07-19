use std::collections::HashMap;

pub mod user_loader;
pub mod rooms_loader;
pub mod participants_loader;
pub mod room_message_loader;

pub type LoadResult<K, T> = anyhow::Result<HashMap<K, T>>;