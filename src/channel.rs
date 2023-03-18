use crate::command::ChannelMode;
use std::collections::HashSet;
pub struct ChannelMeta {
    name: String,
    modes: Vec<ChannelMode>,
    /// a vector of user ids
    users: HashSet<u64>,
    topic: Option<String>,
    limit: Option<u32>,
}

impl ChannelMeta {
    pub fn topic(&self) -> &Option<String> {
        &self.topic
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn new(name: String, user: u64) -> Self {
        let mut users = HashSet::new();
        users.insert(user);
        Self {
            name,
            modes: Vec::new(),
            users,
            topic: None,
            limit: None
        }
    }

    /// for now this doesn't perform any substantial checks
    /// it just allows users in
    pub fn join(&mut self, user: u64) {
        self.users.insert(user);
    }
    pub fn leave(&mut self, user: u64) {
        self.users.remove(&user);
    }
}