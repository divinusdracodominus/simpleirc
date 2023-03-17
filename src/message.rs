use crate::command::{CommandParseError, Command};

#[derive(Debug, Clone)]
pub struct Message {
    // prefix stores the origin of a message
    prefix: Option<String>,
    command: Command,
    trailing: Option<String>,
    raw: String,
}



impl Message {
    pub fn parse(mut message_str: String) -> Result<Self, CommandParseError> {
        // there is a prefix so it should be extracted
        let prefix: Option<String> = if message_str.chars().nth(0) == Some(':') {
            let prefix_offset = match message_str.find(" ") {
                Some(offset) => offset,
                None => return Err(CommandParseError::PrefixOnly(message_str)),
            };
    
            let mut prefix: String = message_str.drain(0..prefix_offset+1).skip(1).collect();
            prefix.pop();
            Some(prefix)
        }else {
            None
        };

        let command = match Command::parse(&message_str) {
            Ok(command) => command,
            Err(e) => {
                return Err(e);
            },
        };

        let (command, trailing, raw_text) = if command != Command::RAW {

            let remainder = message_str.split(':').collect::<Vec<&str>>();
            let trailing = match remainder.get(1) {
                Some(value) => Some(value.to_string()),
                None => None,
            };

            let command_str = match remainder.get(0) {
                Some(command_str) => command_str,
                None => return Err(CommandParseError::NoCommandFound(message_str)),
            };

            let command = match Command::parse(command_str) {
                Ok(command) => command,
                Err(e) => {
                    return Err(e);
                },
            };
            (command, trailing, command_str.to_string())
        }else{
            (command, None, message_str)
        };

        Ok(Self {
            prefix,
            command,
            trailing,
            raw: raw_text,
        })
    }

    pub fn prefix(&self) -> Option<&str> {
        match &self.prefix {
            Some(prefix) => return Some(prefix.as_str()),
            None => return None,
        }
    }

    pub fn raw_message(&self) -> &str {
        self.raw.as_str()
    } 

    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[test]
async fn raw_message_test() {
    let message_str = ":cardinal@localhost this is a message test".to_string();
    let message = Message::parse(message_str).unwrap();
    assert_eq!(message.command(), &Command::RAW);
    
    assert_eq!(message.prefix(), Some("cardinal@localhost"));
    assert_eq!(message.raw_message(), "this is a message test");

    let second_str = "this is another message".to_string();
    let second_message = Message::parse(second_str).unwrap();
    assert_eq!(second_message.command(), &Command::RAW);

    let third_str = "JOIN Welcome,myroom".to_string();
    let third = Message::parse(third_str).unwrap();
    assert!(third.command() != &Command::RAW);

    match third.command() {
        Command::JOIN(channels, keys, realname) => {
            assert_eq!(channels, &vec!["Welcome".to_string(), "myroom".to_string()]);
        },
        _ => panic!("unexpected type"),
    }
}