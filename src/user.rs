use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct User{
    username: String,
    hostname: String,
    realname: String,
    server: String,
    nick: Option<String>,
}

impl User {
    pub fn new(
        username: String,
        hostname: String,
        realname: String,
        server: String
    ) -> Self {
        Self {
            username,
            hostname,
            realname,
            server,
            nick: None,
        }
    }

    pub fn nick(&mut self, nick: String) -> &mut Self {
        self.nick = Some(nick);
        self
    }

    pub fn gen_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn sig(&self) -> String {
        if let Some(nick) = &self.nick {
            format!("{}!{}@{}", nick, self.username, self.hostname)
        }else{
            format!("{}@{}", self.username, self.hostname)
        }
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn parse_sig(sig: &str) -> (Option<String>, String, String) {
        /*if let Some(nick_offset) = sig.find("!") {
            None
        }else{
            None
        };*/

        unimplemented!()
    }
}