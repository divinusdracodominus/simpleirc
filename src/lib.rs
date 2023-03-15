#[macro_use]extern crate serde_derive;
#[macro_use]extern crate tokio;
use structopt::StructOpt;

pub mod command;

pub enum ChannelOp {
    KICK,
    MODE,
    INVITE,
    TOPIC
}

#[derive(StructOpt)]
pub struct Args {
    /// contains the address and port either to bind to
    /// or to connect to depending on wether or not running server or client
    #[structopt(short, long)]
    pub address: String,
    
}

pub enum Error {
    ServerDisconnect,
    ClientDisconnect,
}