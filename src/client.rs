/// this modules provides utilities
/// for the client binary program in order to keep
/// the binary itself relatively clean

use crate::command::*;

pub struct CmdOperator {
    channel: String,
}

impl CmdOperator {
    pub fn default() -> Self {
        Self {
            channel: String::from("Welcome"),
        }
    }
    pub fn read_input() -> Result<String, CommandParseError> {
        let mut input = String::new();
        match std::io::stdin()
        .read_line(&mut input) {
            Ok(_) => {},
            Err(e) => return Err(CommandParseError::NoCommandFound(input)),
        }
        Self::parse_input(input)
    }
    pub fn parse_input(input: String) -> Result<String, CommandParseError> {
        // what has been entered is a command
        let firstchar = input.chars().nth(0);
        let cmdendopt = input.find(' ');
        if firstchar == Some('/') {
            if let Some(cmdend) = cmdendopt {
                let mut invec = input.chars().collect::<Vec<char>>();
                let mut newstring = String::new();
                for i in 1..invec.len() {
                    let mut val = invec[i];
                    if i < cmdend {
                        val = val.to_ascii_uppercase();
                    }
                    newstring.push(val);
                }
                
                Ok(newstring.trim().to_string())
            }else{
                let mut invec = input.chars().collect::<Vec<char>>();
                let mut newstring = String::new();
                for i in 1..invec.len() {
                    let mut val = invec[i];
                    val = val.to_ascii_uppercase();
                    newstring.push(val);
                }
                Ok(newstring.trim().to_string())
            }
        }else{
            // its just a message, forward it right along
            Ok(input)
        }
    }
}

#[test]
async fn convert_input() {
    let input = "/join cardinal".to_string();
    let parsed = CmdOperator::parse_input(input).unwrap();
    assert_eq!(parsed.as_str(), String::from("JOIN cardinal"));
    let list = "/list".to_string();
    let list_parsed = CmdOperator::parse_input(list).unwrap();
    assert_eq!(list_parsed.as_str(), "LIST".to_string());
}