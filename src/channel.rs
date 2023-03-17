use crate::command::ChannelMode;
pub struct ChannelMeta {
    name: String,
    modes: Vec<ChannelMode>,
    /// a vector of user ids
    users: Vec<u64>,
    topic: Option<String>,
    limit: Option<u32>,
}

impl ChannelMeta {
    pub fn topic(&mut self, topic: String) -> &mut Self {
        self.topic = Some(topic);
        self
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn new(name: String, user: u64) -> Self {
        let mut users = Vec::new();
        users.push(user);
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
        self.users.push(user);
    }
}