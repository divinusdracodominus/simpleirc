use crate::command::ChannelMode;
pub struct Channel {
    modes: Vec<ChannelMode>,
    /// a vector of user ids
    users: Vec<String>,
}