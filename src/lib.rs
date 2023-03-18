#[macro_use]extern crate serde_derive;
#[macro_use]extern crate tokio;
#[macro_use]extern crate err_derive;
use structopt::StructOpt;

pub mod command;
pub mod channel;
pub mod message;
pub mod user;
pub mod client;

pub enum ChannelOp {
    KICK,
    MODE,
    INVITE,
    TOPIC
}

/// this is the servers argument builder
#[derive(StructOpt)]
pub struct Args {
    /// contains the address and port either to bind to
    /// or to connect to depending on wether or not running server or client
    #[structopt(short, long)]
    pub address: String,
}

/// this is the clients argument builder, which has a few more required arguments
#[derive(StructOpt)]
pub struct ClientArgs {
    /// contains the address and port either to bind to
    /// or to connect to depending on wether or not running server or client
    #[structopt(short, long)]
    pub address: String,
    #[structopt(short, long)]
    pub hostname: String,
    /// only required for clients, this specifies the username used to connect to the server
    #[structopt(short, long)]
    pub username: String,
    /// only required for clients this specifies the real name used for example "Julian Lazaras"
    #[structopt(short, long)]
    pub realname: String,
    /// completely optional specifies the nickname to use when connecting to the server
    #[structopt(short, long)]
    pub nick: Option<String>,
}

#[derive(Debug, Error)]
pub enum IrcError {
    #[error(display = "server disconnected with error: {:?}", _0)]
    ServerDisconnect(std::io::Error),
    #[error(display = "client disconnected with error: {:?}", _0)]
    ClientDisconnect(std::io::Error),
    #[error(display = "{:?}", _0)]
    CommandParse(crate::command::CommandParseError),
    #[error(display = "the first command sent to the server must be the USER command")]
    MissingUser,
    #[error(display = "DoS potential, this error occures when a client doesn't respond to a ping with pong")]
    DoSWarning,
    #[error(display = "couldn't parse incoming message as utf8")]
    Utf8Error,
}

impl From<std::io::Error> for IrcError {
    fn from(error: std::io::Error) -> IrcError {
        IrcError::ServerDisconnect(error)
    }
}

use tokio::io::AsyncReadExt;
pub async fn read_message<S: AsyncReadExt + std::marker::Unpin>(stream: &mut S) -> Result<(String, usize), IrcError> {
    
    let mut buffer: [u8;512] = [0;512];
    
    let bytes_read = stream.read(&mut buffer).await?;
    let data = match String::from_utf8(buffer[0..bytes_read].to_vec()) {
        Ok(string) => string,
        Err(_) => return Err(IrcError::Utf8Error),
    };
    Ok((data, bytes_read))
}