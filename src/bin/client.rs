#[macro_use]extern crate tokio;
use tokio::net::{TcpStream};
use networkingirc::Args;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let mut client = match TcpStream::connect(args.address).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("error occured: {}", e);
            std::process::exit(-1);
        }
    };

    
}