
use serde::{ser::Serialize, de::Deserialize};
use std::fmt::Debug;
/// denotes a list of channel flags
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelMode {
    /// o = change operator permissions
    OperatorPrivileges,
    /// p = private channel
    PrivateChannel,
    /// s = secret channel
    SecretChannel,
    /// i = invite only channel
    InviteOnly,
    /// t = topic settable by operator only
    TopicSettability,
    /// n = no messages to channels from outside clients?
    NoOutSideClients,
    /// m = moderated channel
    ModeratedChannel,
    /// l = set user limit
    UserLimit,
    /// b = set a ban mask
    BanSet,
    /// v = toggle the ability to speak on a moderated channel
    VoiceToggle,
    /// k = set a channel key / password
    ChannelPassword,

    /// unrecognized character
    Unknown(char),
}

impl ModeTrait for ChannelMode {
    fn from_char(c: char) -> Self {
        match c {
            'o' => Self::OperatorPrivileges,
            'p' => Self::PrivateChannel,
            's' => Self::SecretChannel,
            'i' => Self::InviteOnly,
            't' => Self::TopicSettability,
            'n' => Self::NoOutSideClients,
            'm' => Self::ModeratedChannel,
            'l' => Self::UserLimit,
            'b' => Self::BanSet,
            'v' => Self::VoiceToggle,
            'k' => Self::ChannelPassword,
            _ => Self::Unknown(c)
        }
    }
}

/// +o should be ignored by server (client should be able to make themselves an operator)
/// but -o is acceptable
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserMode {
    /// i = invisible
    Invisible,
    /// s = receives server notices
    NoticeList,
    /// w = user receives wallops (will have to check what a wallop is)
    Wallops,
    /// o = operator: user is an operator
    Operator,
    /// a = away
    Away,
    /// r = restricted
    Restricted,
    /// x = user host is hidden
    MaskedHost,
    /// unknown character
    Unknown(char),
}

impl ModeTrait for UserMode {
    fn from_char(c: char) -> Self {
        match c {
            'i' => Self::Invisible,
            's' => Self::NoticeList,
            'w' => Self::Wallops,
            'o' => Self::Operator,
            'a' => Self::Away,
            'r' => Self::Restricted,
            'x' => Self::MaskedHost,
            _ => Self::Unknown(c)
        }
    }
}

pub trait ModeTrait: Debug + Clone + Serialize + PartialEq{
    fn from_char(c: char) -> Self;
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Mode<T: ModeTrait> {
    Add(T),
    Sub(T),
}

// PLEASE NOTE: the irc crates irc-proto/src/command.rs
// was heavily references in creating this type
// see https://github.com/aatxe/irc/blob/develop/irc-proto/src/command.rs

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    // 3.1 Connection Registration
    /// PASS :password
    PASS(String),
    /// NICK :nickname
    NICK(String),
    /// USER user mode * :realname
    USER(String, String, String),
    /// OPER name :password
    OPER(String, String),
    /// MODE nickname modes
    UserMODE(String, Vec<Mode<UserMode>>),
    /// SERVICE nickname reserved distribution type reserved :info
    SERVICE(String, String, String, String, String, String),
    /// QUIT :comment
    QUIT(Option<String>),
    /// SQUIT server :comment
    SQUIT(String, String),

    // 3.2 Channel operations
    /// JOIN chanlist [chankeys] :[Real name]
    JOIN(String, Option<String>, Option<String>),
    /// PART chanlist :[comment]
    PART(String, Option<String>),
    
    /// set channel modes
    ChannelMode(String, Vec<Mode<ChannelMode>>),
    
    /// set channel topic :[topic]
    TOPIC(String, Option<String>),
    
    /// NAMES [chanlist :[target]]
    NAMES(Option<String>, Option<String>),
    
    /// LIST [chanlist :[target]]
    LIST(Option<String>, Option<String>),
    
    /// INVITE nickname channel
    INVITE(String, String),
    
    /// KICK chanlist userlist :[comment]
    KICK(String, String, Option<String>),

    // 3.3 Sending messages
    /// PRIVMSG msgtarget :message
    ///
    /// ## Responding to a `PRIVMSG`
    ///
    /// When responding to a message, it is not sufficient to simply copy the message target
    /// (msgtarget). This will work just fine for responding to messages in channels where the
    /// target is the same for all participants. However, when the message is sent directly to a
    /// user, this target will be that client's username, and responding to that same target will
    /// actually mean sending itself a response. In such a case, you should instead respond to the
    /// user sending the message as specified in the message prefix. Since this is a common
    /// pattern, there is a utility function
    /// [`Message::response_target`](../message/struct.Message.html#method.response_target)
    /// which is used for this exact purpose.
    PRIVMSG(String, String),
    /// NOTICE msgtarget :message
    ///
    /// ## Responding to a `NOTICE`
    ///
    /// When responding to a notice, it is not sufficient to simply copy the message target
    /// (msgtarget). This will work just fine for responding to messages in channels where the
    /// target is the same for all participants. However, when the message is sent directly to a
    /// user, this target will be that client's username, and responding to that same target will
    /// actually mean sending itself a response. In such a case, you should instead respond to the
    /// user sending the message as specified in the message prefix. Since this is a common
    /// pattern, there is a utility function
    /// [`Message::response_target`](../message/struct.Message.html#method.response_target)
    /// which is used for this exact purpose.
    NOTICE(String, String),

    // 3.4 Server queries and commands
    /// MOTD :[target]
    MOTD(Option<String>),
    /// LUSERS [mask :[target]]
    LUSERS(Option<String>, Option<String>),
    /// VERSION :[target]
    VERSION(Option<String>),
    /// STATS [query :[target]]
    STATS(Option<String>, Option<String>),
    /// LINKS [[remote server] server :mask]
    LINKS(Option<String>, Option<String>),
    /// TIME :[target]
    TIME(Option<String>),
    /// CONNECT target server port :[remote server]
    CONNECT(String, String, Option<String>),
    /// TRACE :[target]
    TRACE(Option<String>),
    /// ADMIN :[target]
    ADMIN(Option<String>),
    /// INFO :[target]
    INFO(Option<String>),

    // 3.5 Service Query and Commands
    /// SERVLIST [mask :[type]]
    SERVLIST(Option<String>, Option<String>),
    /// SQUERY servicename text
    SQUERY(String, String),

    // 3.6 User based queries
    /// WHO [mask ["o"]]
    WHO(Option<String>, Option<bool>),
    /// WHOIS [target] masklist
    WHOIS(Option<String>, String),
    /// WHOWAS nicklist [count :[target]]
    WHOWAS(String, Option<String>, Option<String>),

    // 3.7 Miscellaneous messages
    /// KILL nickname :comment
    KILL(String, String),
    /// PING server1 :[server2]
    PING(String, Option<String>),
    /// PONG server :[server2]
    PONG(String, Option<String>),
    /// ERROR :message
    ERROR(String),

    // 4 Optional Features
    /// AWAY :[message]
    AWAY(Option<String>),
    /// REHASH
    REHASH,
    /// DIE
    DIE,
    /// RESTART
    RESTART,
    /// SUMMON user [target :[channel]]
    SUMMON(String, Option<String>, Option<String>),
    /// USERS :[target]
    USERS(Option<String>),
    /// WALLOPS :Text to be sent
    WALLOPS(String),
    /// USERHOST space-separated nicklist
    USERHOST(Vec<String>),
    /// ISON space-separated nicklist
    ISON(Vec<String>),

    // Default option.
    /// An IRC response code with arguments and optional suffix.
    //Response(Response, Vec<String>),
    /// A raw IRC command unknown to the crate.
    Raw(String, Vec<String>),
}