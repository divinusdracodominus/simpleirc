#[macro_use]extern crate tokio;
use tokio::net::{TcpStream};
use networkingirc::Args;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let mut listener = TcpListener::bind(args.address).await.unwrap();

    /// main event loop to listen for incoming connections
    loop {
        match listener.accept().await {
            Ok((_socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

pub fn launch_client_thread(stream: TcpStream, addr: SocketAddr) {

}