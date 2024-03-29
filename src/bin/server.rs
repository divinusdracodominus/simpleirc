//#[macro_use]extern crate tokio;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tokio::task;

use networkingirc::Args;
use networkingirc::message::Message;
use networkingirc::channel::ChannelMeta;
use networkingirc::command::Command;
use networkingirc::user::User;
use networkingirc::read_message;
use structopt::StructOpt;
use std::net::SocketAddr;
use std::sync::Arc;

use networkingirc::IrcError;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let listener = TcpListener::bind(&args.address).await.unwrap();

    println!("successfully open TCPListener on address: {}", args.address);
    // maintains meta data about all channels in the server
    let channels: Arc<RwLock<HashMap<String, ChannelMeta>>> = Arc::new(RwLock::new(HashMap::new()));
    
    let mut messagelist = HashMap::new();
    messagelist.insert(String::from("Welcome"), vec!["server => welcome to the IRC server, you are now in the welcome channel.".to_string()]);

    // maintains a list of messages organized by channel name
    let messages: Arc<RwLock<HashMap<String, Vec<String>>>> = Arc::new(RwLock::new(messagelist));
    
    // list of all users by the hash of the User struct
    let users: Arc<RwLock<HashMap<u64, User>>> = Arc::new(RwLock::new(HashMap::new())); 

    // main event loop to listen for incoming connections
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("new client: {:?}", addr); 
                let chanref = channels.clone();
                let msgref = messages.clone();
                let userref = users.clone();

                let address_clone = args.address.clone();
                // an additional task is spawned here to handle the initial handshake
                task::spawn(async move {
                    launch_client_listener(chanref, msgref, userref, stream, addr, address_clone).await.unwrap();
                });
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn launch_client_listener(
    channels: Arc<RwLock<HashMap<String, ChannelMeta>>>,
    messages: Arc<RwLock<HashMap<String, Vec<String>>>>,
    users: Arc<RwLock<HashMap<u64, User>>>,
    mut stream: TcpStream,
    addr: SocketAddr,
    address: String,
) -> Result<(), IrcError> {
    println!("entered launch client listener function");
    
    let (data, bytes_read) = read_message(&mut stream).await?;

    println!("read in data: {}", data);
    let user = match Message::parse(data) {
        Ok(message) => {
            println!("received message from new client handshake now");
            match message.command() {
                Command::USER(username, hostname, server, realname) => {
                    User::new(username.to_string(),hostname.to_string(),realname.to_string(),server.to_string())
                },
                _ => {
                    stream.write(b"392 the first command should be USER \r\n").await.unwrap();
                    stream.shutdown().await.unwrap();
                    return Err(IrcError::MissingUser)
                },
            }
        },
        Err(e) => return Err(IrcError::CommandParse(e)),
    };

    {
        let mut write = users.write().await;
        write.insert(user.gen_hash(), user.clone());
        std::mem::drop(write);
    }

    println!("client incoming request received from: {} responding with ping", addr);
    stream.write(format!("PING {} :12345", address).as_bytes()).await?;

    let (pong_read, _) = read_message(&mut stream).await?;
    match Message::parse(pong_read) {
        Ok(message) => {
            match message.command() {
                Command::PONG(server, _) => {
                    println!("received pong from server: {}", server);
                },
                _ => {
                    stream.write(b"392 the second command should be PONG \r\n").await.unwrap();
                    stream.shutdown().await?;
                    return Err(IrcError::DoSWarning)
                },
            }
        },
        Err(e) => return Err(IrcError::CommandParse(e)),
    };
    println!("client connected from address: {} with username: {}", addr, user.username());
    // I'm debating having message meta data sent in the trailing as JSON

    // keeps track of which was the last read message for the channel
    // so only new messages get sent to the client
    //let mut last_read = HashMap::new();
    // however this increases the complexity of client code so for now is omitted
    
    let mut current_channel = String::from("Welcome");
    join_channel(&channels, &current_channel, user.gen_hash()).await;
    display_messages(&current_channel, &messages, &mut stream).await?;

    loop {
        let (cmddata, bytes_read) = read_message(&mut stream).await?;
        if bytes_read == 0 {
            println!("client has disconnected");
            return Ok(());
        }
        match Message::parse(cmddata) {
            Ok(message) => {
                println!("raw message: {:?}", message);
                match message.command() {
                    Command::RAW => {
                        put_message(&current_channel, message.raw_message(), &messages).await;
                    },
                    Command::JOIN(channellist, _keys, _) => {
                        println!("join command received from client");
                        if channellist.len() > 0 {
                            for channel in channellist.iter() {
                                println!("channel: {}", channel);
                                let created = join_channel(&channels, &channel, user.gen_hash()).await;
                                if created {
                                    create_message_board(&channel, &messages).await;
                                }
                            }
                            current_channel = channellist.get(0).unwrap().clone();
                            display_messages(&current_channel, &messages, &mut stream).await?;
                        }
                    },
                    Command::LIST(channelset, _server) => {
                        println!("LIST command invoked with: {:?} querylist", channelset);
                        if channelset.len() == 0 {
                            list_channels(&channels, &mut stream).await?;
                        }else{
                            list_topics(&channels, &channelset, &mut stream).await?;
                        }
                    },
                    Command::NAMES(channellist, _server) => {
                        if channellist.len() == 0 {
                            list_all_users(&users, &mut stream).await?;
                        }else{
                            
                        }
                    },
                    Command::PART(channellist, _) => {
                        leave_channels(&channels, channellist, user.gen_hash()).await;
                    },
                    Command::QUIT(_) => {
                        stream.shutdown().await?;
                        return Ok(());
                    },
                    _ => {},
                }
            },
            Err(e) => {},
        }
    }

    Ok(())
}

async fn leave_channels(
    channels: &Arc<RwLock<HashMap<String, ChannelMeta>>>,
    channellist: &Vec<String>,
    user: u64
) {
    let mut write = channels.write().await;
    for chn in channellist.iter() {
        if let Some(room) = write.get_mut(chn) {
            room.leave(user);
        }
    }
    
}

async fn list_all_users(users: &Arc<RwLock<HashMap<u64, User>>>, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let read = users.read().await;
    let mut outstring = String::new();
    for (id, user) in read.iter() {
        outstring.push_str(&format!("{}\n", user.nickname().as_ref().unwrap_or(&user.username().to_string())));
    }
    std::mem::drop(read);
    stream.write(outstring.as_bytes()).await?;
    Ok(())
}

async fn list_channels(
    channels: &Arc<RwLock<HashMap<String, ChannelMeta>>>,
    stream: &mut TcpStream
) -> Result<(), std::io::Error> {
    let channellist = channels.read().await;
    let mut outstring = String::new();
    for (channel, _) in channellist.iter() {
        outstring.push_str(&format!("{}\n", channel));
    }
    std::mem::drop(channellist);
    stream.write(outstring.as_bytes()).await?;
    Ok(())
}

async fn list_topics(
    channels: &Arc<RwLock<HashMap<String, ChannelMeta>>>, 
    querylist: &Vec<String>,
    stream: &mut TcpStream) -> Result<(), std::io::Error>{
        let channellist = channels.read().await;
        let mut outstring = String::new();
        for query in querylist.iter() {
            if let Some(meta) = channellist.get(query) {
                let chn = query.clone();
                outstring.push_str(&format!("{}\n", &meta.topic().as_ref().unwrap_or(&chn)));
            }
        }
        std::mem::drop(channellist);
        stream.write(outstring.as_bytes()).await?;
        Ok(())
}

async fn put_message(channel: &str, message: &str, messages: &Arc<RwLock<HashMap<String, Vec<String>>>>) {
    let mut write = messages.write().await;
    write.get_mut(channel).unwrap().push(message.to_string());
}

async fn create_message_board(channel: &str, messages: &Arc<RwLock<HashMap<String, Vec<String>>>>) {
    let start_msg = vec![format!("server => this is the begining of: {}", channel)];
    let mut write = messages.write().await;
    let contains = write.contains_key(channel);
    if !contains {
        write.insert(channel.to_string(), start_msg);
    }
}

/// this function retruns a boolean indicating wether or not a channel was created
/// true means a channel was created, false means the channel already existed
async fn join_channel(
    channels: &Arc<RwLock<HashMap<String, ChannelMeta>>>,
    name: &str,
    user: u64,
) -> bool {
    let mut write_lock = channels.write().await;
    if let Some(channel_ref) = write_lock.get_mut(name) {
        channel_ref.join(user);
        false
    }else{
        let channel = ChannelMeta::new(name.to_string(), user);
        write_lock.insert(name.to_string(), channel);
        true
    }
}

/// sends messages to the client upon joining a room
async fn display_messages(
    channel: &str, 
    messages: &Arc<RwLock<HashMap<String, Vec<String>>>>,
    stream: &mut TcpStream
) -> Result<(), std::io::Error> {
    let read = messages.read().await;
    match read.get(channel) {
        Some(message_list) => {
            for message in message_list.iter() {
                stream.write(format!("{}\n", message).as_bytes()).await?;
            }
        },
        None => {},
    }
    Ok(())
}