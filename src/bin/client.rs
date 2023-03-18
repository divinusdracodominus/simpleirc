#[macro_use]extern crate tokio;
use tokio::net::{TcpStream};
use std::net::SocketAddr;
use std::str::FromStr;
use networkingirc::ClientArgs;
use structopt::StructOpt;


use tokio::sync::mpsc::*;
use tokio::task;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use networkingirc::read_message;
use networkingirc::message::Message;
use networkingirc::command::Command;
use networkingirc::client::*;

#[tokio::main]
async fn main() {
    let args = ClientArgs::from_args();
    let mut stream = match TcpStream::connect(&args.address).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("error occured: {}", e);
            std::process::exit(-1);
        }
    };

    stream.write(format!("USER {} {} {} {}", args.username, args.hostname, SocketAddr::from_str(&args.address).unwrap().ip(), args.realname).as_bytes()).await.unwrap();
    println!("sent join command");
    let (mut sender, mut receiver): (Sender<String>, Receiver<String>) = channel(100);
    let (mut read, mut write) = stream.into_split();

    //let address = args.address.clone();

    let read_sender = sender.clone();
    task::spawn(async move {
        loop {
            
            let (data, bytes_read) = match read_message(&mut read).await {
                Ok(bytes) => bytes,
                Err(_) => {
                    eprintln!("server connection closed");
                    std::process::exit(-1);
                },
            };

            if bytes_read == 0 {
                println!("server has terminated the connection exiting now...");
                std::process::exit(0);
                //break;
            }

            

            let message = Message::parse(data).unwrap();
            match message.command() {
                Command::RAW => {
                    print!("{}", message.raw_message());
                },
                Command::PING(server, _) => {
                    println!("received ping from server: {} answering with pong", server);
                    read_sender.send(format!("PONG {} :12345", args.address)).await.unwrap();
                    
                },
                _ => {},
            }
        }
    });

    task::spawn(async move {
        loop {
            let message = match receiver.recv().await {
                Some(value) => value,
                None => break,
            };

            write.write(message.as_bytes()).await;
        }
    });
    
    let operator = CmdOperator::default();

    loop {
        //print!("=>");
        let input = CmdOperator::read_input().unwrap();
        sender.send(input.clone()).await;
        if input == "QUIT" {
            //stream.shutdown().await.unwrap();
            std::process::exit(0);
        }
    }
}